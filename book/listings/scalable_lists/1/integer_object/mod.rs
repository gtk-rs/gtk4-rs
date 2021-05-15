mod imp;

use glib::Object;
use gtk::glib;

// ANCHOR: integer_object
glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

impl From<i32> for IntegerObject {
    fn from(number: i32) -> Self {
        Object::new(&[("number", &number)]).unwrap()
    }
}
// ANCHOR_END: integer_object
