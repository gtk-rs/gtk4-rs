use glib::{ParamFlags, ParamSpec, ParamSpecString, Value};
use gtk::gio;
use gtk::glib;
use gtk::glib::ParamSpecObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use std::cell::RefCell;

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

// Trait shared by all GObjects
impl ObjectImpl for CollectionObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::new(
                    // Name
                    "title",
                    // Nickname
                    "title",
                    // Short description
                    "title",
                    // Default value
                    None,
                    // The property can be read and written to
                    ParamFlags::READWRITE,
                ),
                ParamSpecObject::new(
                    // Name
                    "tasks",
                    // Nickname
                    "tasks",
                    // Short description
                    "tasks",
                    // Object type
                    gio::ListStore::static_type(),
                    // The property can be read and written to
                    ParamFlags::READWRITE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(
        &self,
        _obj: &Self::Type,
        _id: usize,
        value: &Value,
        pspec: &ParamSpec,
    ) {
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

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "title" => self.title.borrow().to_value(),
            "tasks" => self.tasks.get().expect("Could not get tasks.").to_value(),
            _ => unimplemented!(),
        }
    }
}
