use glib::subclass::prelude::*;
use gtk::{
    glib::{self, ParamSpec, Properties, Value},
    prelude::*,
};
use std::cell::{Cell, RefCell};

// The actual data structure that stores our values. This is not accessible
// directly from the outside.
#[derive(Default, Properties)]
#[properties(wrapper_type = super::RowData)]
pub struct RowData {
    #[property(get, set)]
    name: RefCell<Option<String>>,
    #[property(get, set, maximum = 100)]
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
#[glib::derived_properties]
impl ObjectImpl for RowData {}
