mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub(crate)struct TaskObject(ObjectSubclass<imp::TaskObject>);
}

// ANCHOR: impl
impl TaskObject {
    pub(crate) fn new(completed: bool, content: String) -> Self {
        Object::builder()
            .property("completed", completed)
            .property("content", content)
            .build()
    }

    pub(crate) fn is_completed(&self) -> bool {
        self.imp().data.borrow().completed
    }

    pub(crate) fn task_data(&self) -> TaskData {
        self.imp().data.borrow().clone()
    }

    pub(crate) fn from_task_data(task_data: TaskData) -> Self {
        Self::new(task_data.completed, task_data.content)
    }
}
// ANCHOR_END: impl

// ANCHOR: task_data
#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) struct TaskData {
    pub(crate) completed: bool,
    pub(crate) content: String,
}
// ANCHOR_END: task_data
