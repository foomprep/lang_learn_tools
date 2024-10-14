use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Video, Button, Box as GtkBox, Orientation};
use gio::File;

fn main() {
    let app = Application::builder()
        .application_id("com.tongues.srs")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let file_path = "../../.flashcard/media/The.Black.Tulip.1964.REPACK.720p.BluRay.x264.AAC-[YTS.MX]_2258.02.mp4";
    let video = Video::new();
    
    let file = File::for_path(file_path);
    video.set_file(Some(&file));
    video.set_autoplay(true);

 rust
   use serde_json::Value;
use std::fs;

fn main() {
    let subtitle_path = "../../.flashcard/segments/The.Black.Tulip.1964.REPACK.720p.BluRay.x264.AAC-[YTS.MX]_2258.02.json";
    
    // Read the file
    let file_content = fs::read_to_string(subtitle_path)
        .expect("Failed to read the file");

    // Parse JSON
    let json: Value = serde_json::from_str(&file_content)
        .expect    println!("{:?("Failed to parse JSON");
     json);
    }",  }



    let main_box = GtkBox::new(Orientation::Vertical, 5);
    main_box.append(&video);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("SRS")
        .default_width(800)
        .default_height(600)
        .child(&main_box)
        .build();

    window.present();
}
