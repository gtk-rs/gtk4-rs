use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::TaskData;

// ANCHOR: struct_and_subclass
// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::TaskObject)]
pub(crate) struct TaskObject {
    #[property(name = "completed", get, set, type = bool, member = completed)]
    #[property(name = "content", get, set, type = String, member = content)]
    pub(crate) data: RefCell<TaskData>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TaskObject {
    const NAME: &'static str = "TodoTaskObject";
    type Type = super::TaskObject;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for TaskObject {}
// ANCHOR_END: struct_and_subclass
