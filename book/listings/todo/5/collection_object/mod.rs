mod imp;

use crate::task_object::{TaskData, TaskObject};
use glib::Object;
use gtk::prelude::ListModelExtManual;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct CollectionObject(ObjectSubclass<imp::CollectionObject>);
}

impl CollectionObject {
    pub fn new(title: String, tasks: gio::ListStore) -> Self {
        Object::new(&[("title", &title), ("tasks", &tasks)])
            .expect("Failed to create `CollectionObject`.")
    }

    pub fn tasks(&self) -> gio::ListStore {
        self.imp()
            .tasks
            .get()
            .expect("Could not get tasks.")
            .clone()
    }

    pub fn collection_data(&self) -> CollectionData {
        let title = self.imp().title.borrow().clone();
        let tasks_data = self
            .tasks()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<TaskObject>)
            .map(TaskObject::task_data)
            .collect();

        CollectionData { title, tasks_data }
    }

    pub fn from_collection_data(collection_data: CollectionData) -> Self {
        let title = collection_data.title;
        let tasks_to_extend: Vec<TaskObject> = collection_data
            .tasks_data
            .into_iter()
            .map(TaskObject::from_task_data)
            .collect();

        let tasks = gio::ListStore::new(TaskObject::static_type());
        tasks.extend_from_slice(&tasks_to_extend);

        Self::new(title, tasks)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CollectionData {
    pub title: String,
    pub tasks_data: Vec<TaskData>,
}
