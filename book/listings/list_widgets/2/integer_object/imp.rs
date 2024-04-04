use std::cell::Cell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// ANCHOR: integer_object
// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::IntegerObject)]
pub struct IntegerObject {
    #[property(get, set)]
    number: Cell<i32>,
}
// ANCHOR_END: integer_object

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for IntegerObject {
    const NAME: &'static str = "MyGtkAppIntegerObject";
    type Type = super::IntegerObject;
}

// ANCHOR: object_impl
// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for IntegerObject {}
// ANCHOR_END: object_impl
