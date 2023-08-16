use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use gtk::{gio, glib};
use std::cell::OnceCell;

// ANCHOR: collection_object
// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::CollectionObject)]
pub struct CollectionObject {
    #[property(get, set)]
    pub title: RefCell<String>,
    #[property(get, set)]
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
#[glib::derived_properties]
impl ObjectImpl for CollectionObject {}
