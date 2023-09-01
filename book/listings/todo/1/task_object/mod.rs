mod imp;

use glib::Object;
use gtk::glib;

// ANCHOR: glib_wrapper_and_new
glib::wrapper! {
    pub(crate)struct TaskObject(ObjectSubclass<imp::TaskObject>);
}

impl TaskObject {
    pub(crate) fn new(completed: bool, content: String) -> Self {
        Object::builder()
            .property("completed", completed)
            .property("content", content)
            .build()
    }
}
// ANCHOR_END: glib_wrapper_and_new

// ANCHOR: task_data
#[derive(Default)]
pub(crate) struct TaskData {
    pub(crate) completed: bool,
    pub(crate) content: String,
}
// ANCHOR: task_data
