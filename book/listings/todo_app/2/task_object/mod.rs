mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct TaskObject(ObjectSubclass<imp::TaskObject>);
}

// ANCHOR: impl
impl TaskObject {
    pub fn new(completed: bool, content: String) -> Self {
        Object::new(&[("completed", &completed), ("content", &content)])
            .expect("Failed to create `TaskObject`.")
    }

    pub fn is_completed(&self) -> bool {
        self.imp().data.borrow().completed
    }

    pub fn todo_data(&self) -> TaskData {
        self.imp().data.borrow().clone()
    }
}
// ANCHOR_END: impl

// ANCHOR: derive
#[derive(Default, glib::Variant, Clone)]
pub struct TaskData {
    pub completed: bool,
    pub content: String,
}
// ANCHOR_END: derive
