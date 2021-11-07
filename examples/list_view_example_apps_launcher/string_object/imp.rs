use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;

use super::StringData;

// Object holding the state
#[derive(Default)]
pub struct StringObject {
    pub data: Rc<RefCell<StringData>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for StringObject {
    const NAME: &'static str = "StringObject";
    type Type = super::StringObject;
    type ParentType = glib::Object;
}

// Trait shared by all GObjects
impl ObjectImpl for StringObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpec::new_string(
                // Name
                "string",
                // Nickname
                "string",
                // Short description
                "string",
                // Default value
                None,
                // The property can be read and written to
                ParamFlags::READWRITE,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "string" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().string = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "string" => self.data.borrow().string.to_value(),
            _ => unimplemented!(),
        }
    }
}
