mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;

glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

impl IntegerObject {
    pub fn new(number: i32) -> Self {
        Object::builder().property("number", number).build()
    }

    pub fn increase_number(self) {
        let old_number = self.property::<i32>("number");

        self.set_property("number", old_number + 1);
    }
}
