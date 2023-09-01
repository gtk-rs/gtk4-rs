mod imp;

use glib::Object;
use gtk::glib;

// ANCHOR: integer_object
glib::wrapper! {
    pub(crate)struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

impl IntegerObject {
    pub(crate) fn new(number: i32) -> Self {
        Object::builder().property("number", number).build()
    }
}
// ANCHOR_END: integer_object
