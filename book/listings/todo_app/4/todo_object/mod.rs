mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct TodoObject(ObjectSubclass<imp::TodoObject>);
}

impl TodoObject {
    pub fn new(completed: bool, content: String) -> Self {
        Object::new(&[("completed", &completed), ("content", &content)])
            .expect("Failed to create `TodoObject`.")
    }

    pub fn is_completed(&self) -> bool {
        self.imp().data.borrow().completed
    }

    pub fn todo_data(&self) -> TodoData {
        self.imp().data.borrow().clone()
    }
}

#[derive(Default, glib::Variant, Clone)]
pub struct TodoData {
    pub completed: bool,
    pub content: String,
}
