mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct Metadata(ObjectSubclass<imp::Metadata>);
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

impl Metadata {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}
