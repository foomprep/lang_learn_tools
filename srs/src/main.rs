mod translation;
mod word_box;

use gtk::gdk::{Cursor, Display};
use gtk::{prelude::*, Button, GestureClick, Grid, Text};
use gtk::{Application, ApplicationWindow, Video, Box, Orientation, CssProvider};
use serde::{Deserialize, Serialize};
use word_box::WordBox;
use std::cell::Cell;
use gio::File;
use std::sync::mpsc::{self, Sender};
use std::time::Duration;
use std::{fs, thread};
use std::path::PathBuf;
use rand::thread_rng;
use rand::seq::SliceRandom;
use translation::get_translation;
use glib::clone;

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

fn go_to_next_video(
    next_segment: &Segment,
    left_box: &Box,
    tx: Sender<String>,
) {
    while let Some(child) = left_box.last_child() {
        left_box.remove(&child);
    }
    let video = Video::new();
    let file = File::for_path(&next_segment.media_path);
    video.set_file(Some(&file));
    video.set_autoplay(true);
    left_box.append(&video);

    let word_box = Box::new(Orientation::Horizontal, 5);
    let words = next_segment.text.split(' ');
    for word in words {
        let word = word.to_string();
        let word_text = Text::builder()
            .text(&word)
            .editable(false)
            .build();
        let click_controller = GestureClick::new();
        let tx_clone = tx.clone();
        let language = next_segment.language.clone();
        click_controller.connect_pressed(move |_gesture, _n_press, _x, _y| {
            let tx_second_clone = tx_clone.clone();
            let word = word.clone();
            let language_clone = language.clone();
            thread::spawn(move || {
                let word = word.clone();
                let translation = get_translation(
                    &word,
                    &language_clone,
                    "English",
                ).unwrap();
                let _ = tx_second_clone.send(translation);
            });
        });
        word_text.add_controller(click_controller);
        word_box.append(&word_text);
    }
    left_box.append(&word_box);
}

fn build_ui(app: &Application) {
    // Basic scaffold
    let main_grid = Grid::new();
    main_grid.set_column_homogeneous(true);

    let left_box = Box::new(Orientation::Vertical, 5);
    left_box.add_css_class("large-text");
    let right_box = Box::new(Orientation::Vertical, 5);
    let translation_box = Box::new(Orientation::Vertical, 5);
    right_box.append(&translation_box);
    main_grid.attach(&left_box, 0, 0, 4, 1);
    main_grid.attach(&right_box, 5, 5, 1, 1);

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

    let word_box = WordBox::new();
    let subtitles = values_vec[segment_index.get()].text.to_string();
    let language = values_vec[segment_index.get()].language.to_string();
    word_box.set_content(subtitles, language);
    // let word_box = FlowBox::new();
    // word_box.add_css_class("lg-font");
    // word_box.set_cursor_from_name(Some("pointer"));
    // let subtitles = values_vec[segment_index.get()].text.to_string();
    // let language = values_vec[segment_index.get()].language.to_string();
    // let words = subtitles.split(' ');
    // for word in words {
    //     let word = word.to_string();
    //     let word_text = Text::builder()
    //         .text(&word)
    //         .editable(false)
    //         .build();
    //     let click_controller = GestureClick::new();
    //     let tx_clone = tx.clone();
    //     let language_clone = language.clone();
    //     click_controller.connect_pressed(move |_gesture, _n_press, _x, _y| {
    //         let tx_second_clone = tx_clone.clone();
    //         let word = word.clone();
    //         let language_second_clone = language_clone.clone();
    //         thread::spawn(move || {
    //             let word = word.clone();
    //             let translation = get_translation(
    //                 &word,
    //                 &language_second_clone,
    //                 "English",
    //             ).unwrap();
    //             let _ = tx_second_clone.send(translation);
    //         });
    //     });
    //     word_text.add_controller(click_controller);
    //     word_box.append(&word_text);
    // }
    left_box.append(&word_box.flow_box().unwrap());

    let translation_box_clone = translation_box.clone();
    glib::timeout_add_local(Duration::from_millis(100), move || {
        match rx.try_recv() {
            Ok(translation) => {
                while let Some(child) = translation_box_clone.last_child() {
                    translation_box_clone.remove(&child);
                }
                let current_word = Text::builder()
                    .text(&translation)
                    .editable(false)
                    .build();
                translation_box_clone.append(&current_word); 
            },
            Err(_) => {},
        };
        glib::ControlFlow::Continue
    });

    let next_button = Button::with_label("Next");
    let segment_index_clone = segment_index.clone();
    next_button.connect_clicked(clone!(
        #[strong]
        tx,
        #[weak]
        left_box,
        move |_| {
            let tx_clone = tx.clone();
            segment_index_clone.set(segment_index_clone.get()+1);
            let segment = &values_vec[segment_index_clone.get()];
            go_to_next_video(&segment, &left_box, tx_clone); 
        }
    ));
    right_box.append(&next_button);

    // let remove_button = Button::with_label("Remove");
    // remove_button.connect_clicked(clone!(
    //     let segment = values_vec[segment_index.get()];
    // ));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("SRS")
        .default_width(1000)
        .default_height(1000)
        .child(&main_grid)
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
