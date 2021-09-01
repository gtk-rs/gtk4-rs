mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use serde::{Deserialize, Serialize};

use std::cell::RefCell;
use std::rc::Rc;

glib::wrapper! {
    pub struct TodoObject(ObjectSubclass<imp::TodoObject>);
}

impl TodoObject {
    pub fn new(content: String, completed: bool) -> Self {
        Object::new(&[("content", &content), ("completed", &completed)])
            .expect("Failed to create `TodoObject`.")
    }

    pub fn is_completed(&self) -> bool {
        self.property("completed")
            .expect("The property needs to exist and be readable.")
            .get()
            .expect("The property needs to be of type `bool`.")
    }

    pub fn todo_data(&self) -> Rc<RefCell<TodoData>> {
        let imp = imp::TodoObject::from_instance(self);
        imp.data.clone()
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct TodoData {
    pub completed: bool,
    pub content: String,
}
