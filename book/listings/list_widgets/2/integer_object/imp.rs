use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

use std::cell::Cell;

// ANCHOR: integer_object
// Object holding the state
#[derive(Default)]
pub struct IntegerObject {
    number: Cell<i32>,
}
// ANCHOR_END: integer_object

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for IntegerObject {
    const NAME: &'static str = "MyGtkAppIntegerObject";
    type Type = super::IntegerObject;
    type ParentType = glib::Object;
}

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for IntegerObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpec::new_int(
                // Name
                "number",
                // Nickname
                "number",
                // Short description
                "number",
                // Minimum value
                i32::MIN,
                // Maximum value
                i32::MAX,
                // Default value
                0,
                // The property can be read and written to
                ParamFlags::READWRITE,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "number" => {
                let input_number = value.get().expect("The value needs to be of type `i32`.");
                self.number.replace(input_number);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "number" => self.number.get().to_value(),
            _ => unimplemented!(),
        }
    }
}
// ANCHOR_END: object_impl
