use glib::clone;
use gtk::gdk::Display;
use gtk::{prelude::*, Button};
use gtk::{Application, ApplicationWindow, TextView, Video, Box as GtkBox, Orientation, CssProvider, TextBuffer};
use gio::File;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::fs;
use std::path::PathBuf;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize, Debug)]
struct Segment {
    text: String,
    media_path: String,
    language: String,
}

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

    let subtitles_buffer = TextBuffer::new(None);
    subtitles_buffer.set_text(&values_vec[segment_index.get()].text);
    let subtitles_view = TextView::with_buffer(&subtitles_buffer);
    subtitles_view.add_css_class("text-xl");

    let next_button = Button::with_label("Next");
    next_button.connect_clicked(clone!(
        #[strong]
        subtitles_view,
        #[strong]
        video,
        move |_| {
            segment_index.set(segment_index.get()+1);
            let buffer = TextBuffer::new(None);
            buffer.set_text(&values_vec[segment_index.get()].text);
            subtitles_view.set_buffer(Some(&buffer));
            let file = File::for_path(&values_vec[segment_index.get()].media_path);
            video.set_file(Some(&file));
        }
    ));

    let main_box = GtkBox::new(Orientation::Vertical, 5);
    main_box.append(&video);
    main_box.append(&subtitles_view);
    main_box.append(&next_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("SRS")
        .default_width(800)
        .default_height(600)
        .child(&main_box)
        .build();

    window.present();
}
