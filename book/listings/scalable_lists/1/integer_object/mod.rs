mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;

// ANCHOR: integer_object
glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

impl IntegerObject {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create IntegerObject")
    }

    pub fn from_integer(number: i32) -> Self {
        let integer_object = Self::new();
        integer_object.set_property("number", number).unwrap();
        integer_object
    }
}
// ANCHOR_END: integer_object

impl Default for IntegerObject {
    fn default() -> Self {
        Self::new()
    }
}
