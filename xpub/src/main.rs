mod html_parser;

use clap::Parser;
use std::{io::{self, Read, Write}, path::{Path, PathBuf}};
use std::collections::HashMap;
use zip::{write::{SimpleFileOptions, ZipWriter}, ZipArchive};

use html_parser::wrap_words_in_paragraphs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Path of the file to change")]
    input: String,
    #[arg(short, long, help = "Path of the output file", default_value = "modified.epub")]
    output: String,
    #[arg(short, long, help = "Language of epub file")]
    lang: String,
}

fn main() {
    let args = Args::parse();   

    let input_path = args.input;
    let input_zip_path = if Path::new(&input_path).is_absolute() {
        PathBuf::from(input_path)
    } else {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        current_dir.join(input_path)
    };
 
    let output_path = args.output;
    let output_zip_path = if Path::new(&output_path).is_absolute() {
        PathBuf::from(output_path)
    } else {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        current_dir.join(output_path)
    };

    let _ = modify_files_in_zip(
        &input_zip_path, 
        &output_zip_path,
        wrap_words_in_paragraphs,
        &args.lang,
    );

    println!("Epub modified successfully!");
}

fn modify_files_in_zip(
    input_path: &PathBuf, 
    output_path: &PathBuf,
    modify_fn: fn(&str, &str) -> String,
    language: &str,
) -> io::Result<()> {
    let input = std::fs::File::open(input_path).unwrap();
    let output = std::fs::File::create(output_path).unwrap();
    let mut archive = ZipArchive::new(input)?;

    let mut modified_files: HashMap<String, Vec<u8>> = HashMap::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();
 
        if file_name.ends_with(".xhtml") || file_name.ends_with(".html") {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let modified_contents = modify_fn(&contents, language);
            modified_files.insert(file_name, modified_contents.into_bytes());
        } else {
            // Copy the file as is
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            modified_files.insert(file_name, buffer);
        }
    }

    let mut zip_writer = ZipWriter::new(output);

    for (name, data) in modified_files {
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }

    zip_writer.finish().unwrap();
    Ok(())
}

