mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;

glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

impl IntegerObject {
    pub fn new(number: i32) -> Self {
        Object::new(&[("number", &number)]).expect("Failed to create `IntegerObject`.")
    }

    pub fn increase_number(self) {
        let old_number = self
            .property("number")
            .expect("The property needs to exist and be readable.")
            .get::<i32>()
            .expect("The property needs to be of type `i32`.");

        self.set_property("number", old_number + 1)
            .expect("Could not set property.");
    }
}
