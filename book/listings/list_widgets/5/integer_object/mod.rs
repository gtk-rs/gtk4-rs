mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

impl IntegerObject {
    pub fn new(number: i32) -> Self {
        Object::builder().property("number", number).build()
    }

    pub fn increase_number(self) {
        self.set_number(self.number() + 1);
    }
}
