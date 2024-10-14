use gtk::gdk::Display;
use gtk::predule::*;
use gtk::{Application, ApplicationWindow, TextView, Video, Box as GtkBox, Orientation, CssProvider, TextBuffer};
use gio::File;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn main() {
    let app = Application::builder()
        .application_id("com.tongues.srs")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
rust
    let dir_path = dirs::home_dir().unwrap().join(".flashcard/segments");
    if dir_path.exists() && dir_path.is_dir() {
        let mut entries_vec = vec![];
        let entries = fs::read_dir(dir_path).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path: PathBuf = entry.path();
            if path.is_file() {
                entries_vec.push(path);
            }
        }
        entries_vec.shuffle(&mut rand::thread_rng());
        for path in entries_vec {
            let content = fs::read_to_string(&path).unwrap();
            println!("File: {:?} contains:\n{}", path, content);
        }
    } else {
        println!("Directory does not exist");
    }

    let file_path = "../../.flashcard/media/The.Black.Tulip.1964.REPACK.720p.BluRay.x264.AAC-[YTS.MX]_2258.02.mp4";
    let video = Video::new();
    
    let file = File::for_path(file_path);
    video.set_file(Some(&file));
    video.set_autoplay(true);

    let subtitle_path = "../../.flashcard/segments/The.Black.Tulip.1964.REPACK.720p.BluRay.x264.AAC-[YTS.MX]_2258.02.json";
    
    let file_content = fs::read_to_string(subtitle_path)
        .expect("Failed to read the file");

    let segment_json: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON");
    let subtitles_buffer = TextBuffer::new(None);
    subtitles_buffer.set_text(segment_json["text"].as_str().unwrap_or_default());
    let subtitles_view = TextView::with_buffer(&subtitles_buffer);
    subtitles_view.add_css_class("text-xl");

    let main_box = GtkBox::new(Orientation::Vertical, 5);
    main_box.append(&video);
    main_box.append(&subtitles_view);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("SRS")
        .default_width(800)
        .default_height(600)
        .child(&main_box)
        .build();

    window.present();
}
