use anthropic::client::ClientBuilder;
use anthropic::types::{ContentBlock, Message, MessagesRequestBuilder, Role};
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Video, Box as GtkBox, Orientation, CssProvider, Label};
use gio::File;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::fs;
use std::env;
use std::path::PathBuf;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize, Debug)]
struct Segment {
    text: String,
    media_path: String,
    language: String,
}

async fn get_translation(
    text: &str, 
    source_language: &str,
    target_language: &str,
) -> String {
    let api_key = match env::var("ANTHROPIC_API_KEY") {
        Ok(api_key) => api_key,
        Err(_) => panic!("ANTHROPIC_API_KEY not set."),
    };

    let client = ClientBuilder::default().api_key(api_key).build()?;

    let message = Message { 
        role: Role::User, 
        content: vec![ContentBlock::Text { text: "hello".to_string() }]
    };
    let request = MessagesRequestBuilder::default()
        .model("claude-3-haiku-20240307".to_string())
        .max_tokens::<u16>(400)
        .stream(false)  
        .messages(vec![message])
        .build().unwrap();

    let response = client.messages(request).await.unwrap();
    match response.content.get(0).unwrap() {
        ContentBlock::Text { text } => text.to_string(),
        _ => panic!("Not text.")
    }
}

fn main() {
    let app = Application::builder()
        .application_id("com.tongues.srs")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| {
        build_ui(app);
    });
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
    let segment_index = Cell::<usize>::new(0);
    let dir_path = dirs::home_dir().unwrap().join(".flashcard/segments");
    let mut entries_vec = vec![];
    if dir_path.exists() && dir_path.is_dir() {
      let entries = fs::read_dir(dir_path).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path: PathBuf = entry.path();
            if path.is_file() {
                entries_vec.push(path);
            }
        }
        entries_vec.shuffle(&mut thread_rng());
    } else {
        println!("Directory does not exist");
    }

    let mut values_vec = vec![];
    for path in entries_vec {
        let content = fs::read_to_string(&path).unwrap();
        let segment_json: Segment = serde_json::from_str(&content).expect("Failed to parse JSON");
        values_vec.push(segment_json);
    }

    let video = Video::new();
    let file = File::for_path(&values_vec[segment_index.get()].media_path);
    video.set_file(Some(&file));
    video.set_autoplay(true);

    let word_box = GtkBox::new(Orientation::Horizontal, 10);
    word_box.add_css_class("text-xl");
    let words = values_vec[segment_index.get()].text.split(' ');
    for word in words {
        let word_label = Label::new(Some(word));
        word_box.append(&word_label);
    }

    // let next_button = Button::with_label("Next");
    // next_button.connect_clicked(clone!(
    //     #[strong]
    //     word_box,
    //     #[strong]
    //     video,
    //     move |_| {
    //         segment_index.set(segment_index.get()+1);

    //         let words = values_vec[segment_index.get()].text.split(' ');
    //         for word in words {
    //             let buffer = TextBuffer::builder().text(word).build();
    //             let text_view = TextView::builder().buffer(&buffer).editable(false).build();
    //             word_box.append(&text_view);
    //         }
    //         let file = File::for_path(&values_vec[segment_index.get()].media_path);
    //         video.set_file(Some(&file));
    //     }
    // ));

    let main_box = GtkBox::new(Orientation::Vertical, 5);
    main_box.append(&video);
    main_box.append(&word_box);
    // main_box.append(&next_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("SRS")
        .default_width(800)
        .default_height(800)
        .child(&main_box)
        .build();

    window.present();
}
