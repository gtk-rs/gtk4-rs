mod imp;

use glib::Object;
use gtk::glib;

// ANCHOR: glib_wrapper_and_new
glib::wrapper! {
    pub struct TaskObject(ObjectSubclass<imp::TaskObject>);
}

impl TaskObject {
    pub fn new(completed: bool, content: String) -> Self {
        Object::new(&[("completed", &completed), ("content", &content)])
    }
}
// ANCHOR_END: glib_wrapper_and_new

// ANCHOR: task_data
#[derive(Default)]
pub struct TaskData {
    pub completed: bool,
    pub content: String,
}
// ANCHOR: task_data
