use gtk::gdk::Display;
use gtk::{prelude::*, GestureClick, Text};
use gtk::{Application, ApplicationWindow, Video, Box as GtkBox, Orientation, CssProvider, Label};
use gio::File;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::cell::Cell;
use std::fs;
use std::env;
use std::path::PathBuf;
use reqwest::blocking::Client;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize, Debug)]
struct Segment {
    text: String,
    media_path: String,
    language: String,
}

fn get_translation(text: &str, source_language: &str, target_language: &str) -> Result<String, anyhow::Error> {
    // TODO move to parameters
    let client = reqwest::blocking::Client::new();
    let api_key = std::env::var("ANTHROPIC_API_KEY").expect("Environment variable ANTHROPIC_API_KEY is not set");
  
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2024-01-01")
        .header("content-type", "application/json")
        .json(&json!({
            "model": "claude-3-opus-20240229",
            "max_tokens": 1024,
            "messages": [{
                "role": "user",
                "content": format!(
                    "Translate the following text from {} to {}. Only provide the translation, no explanations:\n\n{}",
                    source_language, target_language, text
                )
            }]
        }))
        .send()?;

    // Extract the translation from the response
rust
    let response_text: serde_json::Value = serde_json::from_str(&response.text()?)?;

    let translation = response_text["content"][0]["text"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to extract translation from response"))?
        .to_string();

    Ok(translation)
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
    let subtitles = values_vec[segment_index.get()].text.to_string();
    let words = subtitles.split(' ');
    for word in words {
        let word = word.to_string();
        let word_text = Text::builder()
            .text("hello")
            .editable(false)
            .build();
        let click_controller = GestureClick::new();
        click_controller.connect_pressed(move |_gesture, _n_press, _x, _y| {
            let word = word.clone();
            glib::spawn_future_local(async move {
                    let word = word.clone();
                    let translation = get_translation(
                        &word,
                        "English",
                        "Spanish",
                    ).await.unwrap();
                    println!("{}", translation);
                }
            );
        });
        word_text.add_controller(click_controller);
        word_box.append(&word_text);
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
