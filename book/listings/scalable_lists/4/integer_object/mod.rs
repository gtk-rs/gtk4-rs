mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;

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

    pub fn increase_number(self) {
        let old_number = self
            .property("number")
            .expect("The property needs to exist and be readable.")
            .get::<i32>()
            .expect("The property needs to be of type `i32`.");

        self.set_property("number", old_number + 1).unwrap();
    }
}

impl Default for IntegerObject {
    fn default() -> Self {
        Self::new()
    }
}
