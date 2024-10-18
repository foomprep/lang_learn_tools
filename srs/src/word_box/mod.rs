use gtk::prelude::*;
use gtk::{FlowBox, Label};
use gtk::glib;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct SubtitleFlowBox {
        pub flow_box: RefCell<Option<FlowBox>>,
        pub subtitles: RefCell<String>,
        pub language: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SubtitleFlowBox {
        const NAME: &'static str = "SubtitleFlowBox";
        type Type = super::SubtitleFlowBox;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for SubtitleFlowBox {
        fn constructed(&self) {
            self.parent_constructed();
            
            let obj = self.obj();
            
            let flow_box = FlowBox::new();
            flow_box.set_valign(gtk::Align::Start);
            flow_box.set_max_children_per_line(30);
            flow_box.set_selection_mode(gtk::SelectionMode::None);
            flow_box.set_row_spacing(10);
            flow_box.set_column_spacing(10);
            
            *self.flow_box.borrow_mut() = Some(flow_box.clone());
            
        }
    }

    impl WidgetImpl for SubtitleFlowBox {}
}

glib::wrapper! {
    pub struct SubtitleFlowBox(ObjectSubclass<imp::SubtitleFlowBox>)
        @extends gtk::Widget;
}

impl SubtitleFlowBox {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_content(&self, subtitles: String, language: String) {
        let imp = self.imp();
        *imp.subtitles.borrow_mut() = subtitles.clone();
        *imp.language.borrow_mut() = language;

        if let Some(flow_box) = imp.flow_box.borrow().as_ref() {
            // Clear existing children
            while let Some(child) = flow_box.first_child() {
                flow_box.remove(&child);
            }

            // Split subtitles into words and create label for each
            for word in subtitles.split_whitespace() {
                let label = Label::new(Some(word));
                label.set_margin_start(5);
                label.set_margin_end(5);
                flow_box.append(&label);
            }
        }
    }

    pub fn flow_box(&self) -> Option<FlowBox> {
        self.imp().flow_box.borrow().clone()
    }
}