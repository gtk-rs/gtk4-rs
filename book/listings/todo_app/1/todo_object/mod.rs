mod imp;

use glib::Object;
use gtk::glib;

// ANCHOR: glib_wrapper_and_new
glib::wrapper! {
    pub struct TodoObject(ObjectSubclass<imp::TodoObject>);
}

impl TodoObject {
    pub fn new(completed: bool, content: String) -> Self {
        Object::new(&[("completed", &completed), ("content", &content)])
            .expect("Failed to create `TodoObject`.")
    }
}
// ANCHOR_END: glib_wrapper_and_new

// ANCHOR: todo_data
#[derive(Default)]
pub struct TodoData {
    pub completed: bool,
    pub content: String,
}
// ANCHOR: todo_data
