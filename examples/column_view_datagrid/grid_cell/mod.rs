mod imp;
use gtk::{glib, subclass::prelude::*};

pub struct Entry {
    pub name: String,
}

glib::wrapper! {
    pub struct GridCell(ObjectSubclass<imp::GridCell>)
        @extends gtk::Widget;
}

impl Default for GridCell {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl GridCell {
    pub fn set_entry(&self, entry: &Entry) {
        self.imp().name.set_text(Some(&entry.name));
    }
}
