mod translation;

use gtk::gdk::Display;
use gtk::{prelude::*, FlowBox, GestureClick, Text};
use gtk::{Application, ApplicationWindow, Video, Box as GtkBox, Orientation, CssProvider};
use gio::File;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::sync::mpsc;
use std::time::Duration;
use std::{fs, thread};
use std::path::PathBuf;
use rand::thread_rng;
use rand::seq::SliceRandom;
use translation::get_translation;

#[derive(Serialize, Deserialize, Debug)]
struct Segment {
    text: String,
    media_path: String,
    language: String,
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
    // Basic scaffold
    let main_box = GtkBox::new(Orientation::Horizontal, 5);
    let left_box = GtkBox::new(Orientation::Vertical, 5);
    left_box.add_css_class("h-3/4");
    left_box.add_css_class("text-xl");
    let right_box = GtkBox::new(Orientation::Vertical, 5);
    right_box.add_css_class("h-1/4");
    main_box.append(&left_box);
    main_box.append(&right_box);

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
    left_box.append(&video);

    let (tx, rx) = mpsc::channel();

    let word_box = FlowBox::new();
    word_box.set_vexpand(true);
    word_box.set_hexpand(true);
    word_box.add_css_class("word");
    let subtitles = values_vec[segment_index.get()].text.to_string();
    let language = values_vec[segment_index.get()].language.to_string();
    let words = subtitles.split(' ');
    for word in words {
        let word = word.to_string();
        let word_text = Text::builder()
            .text(&word)
            .editable(false)
            .build();
        let click_controller = GestureClick::new();
        let tx_clone = tx.clone();
        let language_clone = language.clone();
        click_controller.connect_pressed(move |_gesture, _n_press, _x, _y| {
            let tx_second_clone = tx_clone.clone();
            let word = word.clone();
            let language_second_clone = language_clone.clone();
            thread::spawn(move || {
                let word = word.clone();
                let translation = get_translation(
                    &word,
                    &language_second_clone,
                    "English",
                ).unwrap();
                let _ = tx_second_clone.send(translation);
            });
        });
        word_text.add_controller(click_controller);
        word_box.append(&word_text);
    }
    left_box.append(&word_box);

    glib::timeout_add_local(Duration::from_millis(100), move || {
        match rx.try_recv() {
            Ok(translation) => {
                while let Some(child) = right_box.last_child() {
                    right_box.remove(&child);
                }
                let current_word = Text::builder()
                    .text(&translation)
                    .editable(false)
                    .build();
                right_box.append(&current_word); 
            },
            Err(_) => {},
        };
        glib::ControlFlow::Continue
    });

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


    let window = ApplicationWindow::builder()
        .application(app)
        .title("SRS")
        .default_width(800)
        .default_height(800)
        .child(&main_box)
        .build();

    window.present();
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