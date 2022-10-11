use glib::subclass::prelude::*;
use gtk::{
    glib::{self, ParamSpec, Value},
    prelude::*,
};
use std::cell::{Cell, RefCell};

// The actual data structure that stores our values. This is not accessible
// directly from the outside.
#[derive(Default)]
pub struct RowData {
    name: RefCell<Option<String>>,
    count: Cell<u32>,
}

// Basic declaration of our type for the GObject type system
#[glib::object_subclass]
impl ObjectSubclass for RowData {
    const NAME: &'static str = "RowData";
    type Type = super::RowData;
}

// The ObjectImpl trait provides the setters/getters for GObject properties.
// Here we need to provide the values that are internally stored back to the
// caller, or store whatever new value the caller is providing.
//
// This maps between the GObject properties and our internal storage of the
// corresponding values of the properties.
impl ObjectImpl for RowData {
    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecString::builder("name").build(),
                glib::ParamSpecUInt::builder("count").maximum(100).build(),
            ]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let name = value.get().unwrap();
                self.name.replace(name);
            }
            "count" => {
                let count = value.get().unwrap();
                self.count.replace(count);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "name" => self.name.borrow().to_value(),
            "count" => self.count.get().to_value(),
            _ => unimplemented!(),
        }
    }
}
