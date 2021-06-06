use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CheckButton, Entry};
use once_cell::sync::Lazy;

// Object holding the state
pub struct TodoRow {
    completed_button: CheckButton,
    content_entry: Entry,
}

impl Default for TodoRow {
    fn default() -> Self {
        Self {
            completed_button: CheckButton::new(),
            content_entry: Entry::new(),
        }
    }
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TodoRow {
    const NAME: &'static str = "MyGtkAppTodoRow";
    type Type = super::TodoRow;
    type ParentType = gtk::Box;
}

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for TodoRow {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpec::new_object(
                    // Name
                    "completed-button",
                    // Nickname
                    "completed-button",
                    // Short description
                    "completed-button",
                    // Object Type
                    CheckButton::static_type(),
                    // The property can be read
                    ParamFlags::READABLE,
                ),
                ParamSpec::new_object(
                    // Name
                    "content-entry",
                    // Nickname
                    "content-entry",
                    // Short description
                    "content-entry",
                    // Object Type
                    Entry::static_type(),
                    // The property can be read
                    ParamFlags::READABLE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "completed-button" => self.completed_button.to_value(),
            "content-entry" => self.content_entry.to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        obj.append(&self.completed_button);
        obj.append(&self.content_entry)
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for TodoRow {}

// Trait shared by all boxes
impl BoxImpl for TodoRow {}
