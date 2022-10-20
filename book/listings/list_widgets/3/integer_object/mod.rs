mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;

glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

// ANCHOR: integer_object
impl IntegerObject {
    pub fn new(number: i32) -> Self {
        Object::new(&[("number", &number)])
    }

    pub fn increase_number(self) {
        let old_number = self.property::<i32>("number");
        self.set_property("number", old_number + 1);
    }
}
// ANCHOR_END: integer_object
