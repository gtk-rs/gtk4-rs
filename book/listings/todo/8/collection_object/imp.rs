use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::{ParamSpec, ParamSpecString, Value};
use gtk::glib::ParamSpecObject;
use gtk::{gio, glib};
use once_cell::sync::{Lazy, OnceCell};

// ANCHOR: collection_object
// Object holding the state
#[derive(Default)]
pub struct CollectionObject {
    pub title: RefCell<String>,
    pub tasks: OnceCell<gio::ListStore>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CollectionObject {
    const NAME: &'static str = "TodoCollectionObject";
    type Type = super::CollectionObject;
}
// ANCHOR_END: collection_object

// Trait shared by all GObjects
impl ObjectImpl for CollectionObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("title").build(),
                ParamSpecObject::builder::<gio::ListStore>("tasks").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "title" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.title.replace(input_value);
            }
            "tasks" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `gio::ListStore`.");
                self.tasks.set(input_value).expect("Could not set task");
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "title" => self.title.borrow().to_value(),
            "tasks" => self.tasks.get().expect("Could not get tasks.").to_value(),
            _ => unimplemented!(),
        }
    }
}
