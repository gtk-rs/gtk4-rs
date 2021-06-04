mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct TodoObject(ObjectSubclass<imp::TodoObject>);
}

impl TodoObject {
    pub fn new(content: String, completed: bool) -> Self {
        Object::new(&[("content", &content), ("completed", &completed)]).unwrap()
    }
}
