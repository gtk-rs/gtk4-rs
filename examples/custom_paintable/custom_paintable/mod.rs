mod imp;

use gtk::{gdk, glib};

glib::wrapper! {
    pub struct CustomPaintable(ObjectSubclass<imp::CustomPaintable>) @implements gdk::Paintable;
}

impl Default for CustomPaintable {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomPaintable {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a CustomPaintable")
    }
}
