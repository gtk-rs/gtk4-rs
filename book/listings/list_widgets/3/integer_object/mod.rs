mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub(crate)struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

// ANCHOR: integer_object
impl IntegerObject {
    pub(crate) fn new(number: i32) -> Self {
        Object::builder().property("number", number).build()
    }

    pub(crate) fn increase_number(self) {
        self.set_number(self.number() + 1);
    }
}
// ANCHOR_END: integer_object
