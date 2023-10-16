mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct Metadata(ObjectSubclass<imp::Metadata>);
}

impl Default for Metadata {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl Metadata {
    pub fn new(title: &str) -> Self {
        glib::Object::builder().property("title", title).build()
    }
}
