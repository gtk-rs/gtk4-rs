mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct StringObject(ObjectSubclass<imp::StringObject>);
}

impl StringObject {
    pub fn new(string: String) -> Self {
        Object::new(&[("string", &string)]).expect("Failed to create `StringObject`.")
    }
}

#[derive(Default)]
pub struct StringData {
    pub string: String,
}
