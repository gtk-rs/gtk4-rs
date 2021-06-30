use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use once_cell::sync::Lazy;
use std::cell::RefCell;

// Object holding the state
#[derive(Default)]
pub struct TodoObject {
    pub data: RefCell<super::TodoData>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TodoObject {
    const NAME: &'static str = "MyGtkAppTodoObject";
    type Type = super::TodoObject;
    type ParentType = glib::Object;
}

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for TodoObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpec::new_boolean(
                    // Name
                    "completed",
                    // Nickname
                    "completed",
                    // Short description
                    "completed",
                    // Default value
                    false,
                    // The property can be read and written to
                    ParamFlags::READWRITE,
                ),
                ParamSpec::new_string(
                    // Name
                    "content",
                    // Nickname
                    "content",
                    // Short description
                    "content",
                    // Default value
                    None,
                    // The property can be read and written to
                    ParamFlags::READWRITE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "completed" => {
                let input_value = value.get().expect("The value needs to be of type `bool`.");
                self.data.borrow_mut().completed = input_value;
            }
            "content" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().content = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "completed" => self.data.borrow().completed.to_value(),
            "content" => self.data.borrow().content.to_value(),
            _ => unimplemented!(),
        }
    }
}
// ANCHOR_END: object_impl
