mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct TaskObject(ObjectSubclass<imp::TaskObject>);
}

impl TaskObject {
    pub fn new(completed: bool, content: String) -> Self {
        Object::new(&[("completed", &completed), ("content", &content)])
    }

    pub fn is_completed(&self) -> bool {
        self.imp().data.borrow().completed
    }

    pub fn task_data(&self) -> TaskData {
        self.imp().data.borrow().clone()
    }

    pub fn from_task_data(task_data: TaskData) -> Self {
        Self::new(task_data.completed, task_data.content)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TaskData {
    pub completed: bool,
    pub content: String,
}
