mod imp;

use glib::Object;
use gtk::glib;

// ANCHOR: glib_wrapper
glib::wrapper! {
    pub struct TodoObject(ObjectSubclass<imp::TodoObject>);
}
// ANCHOR_END: glib_wrapper

impl TodoObject {
    pub fn new(content: String, completed: bool) -> Self {
        Object::new(&[("content", &content), ("completed", &completed)])
            .expect("Failed to create `TodoObject`.")
    }
}

// ANCHOR: todo_data
#[derive(Default)]
pub struct TodoData {
    pub completed: bool,
    pub content: String,
}
// ANCHOR: todo_data
