use crate::todo_object::TodoObject;
use glib::{Binding, BindingFlags, ParamFlags, ParamSpec, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use gtk::{CheckButton, Label};
use once_cell::sync::Lazy;
use pango::{AttrList, Attribute};
use std::cell::RefCell;

// Object holding the state
pub struct TodoRow {
    completed_button: CheckButton,
    content_label: Label,
    bindings: RefCell<Vec<Binding>>,
}

impl Default for TodoRow {
    fn default() -> Self {
        Self {
            completed_button: CheckButton::builder()
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build(),
            content_label: Label::builder()
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build(),
            bindings: RefCell::default(),
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
                    "content-label",
                    // Nickname
                    "content-label",
                    // Short description
                    "content-label",
                    // Object Type
                    Label::static_type(),
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
            "content-label" => self.content_label.to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        obj.append(&self.completed_button);
        obj.append(&self.content_label)
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for TodoRow {}

// Trait shared by all boxes
impl BoxImpl for TodoRow {}

impl TodoRow {
    pub fn bind_item(&self, item: &TodoObject) {
        let completed_button_binding = item
            .bind_property("completed", &self.completed_button, "active")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build()
            .unwrap();
        self.bindings.borrow_mut().push(completed_button_binding);

        let content_label_binding = item
            .bind_property("content", &self.content_label, "label")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build()
            .unwrap();
        self.bindings.borrow_mut().push(content_label_binding);

        let completed_label_binding = item
            .bind_property("completed", &self.content_label, "attributes")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::DEFAULT)
            .transform_to(|_, completed_value| {
                let attribute_list = AttrList::new();
                let completed = completed_value
                    .get::<bool>()
                    .expect("The value needs to be of type `bool`.");
                if completed {
                    let attribute = Attribute::new_strikethrough(true);
                    attribute_list.insert(attribute);
                }
                Some(attribute_list.to_value())
            })
            .build()
            .unwrap();
        self.bindings.borrow_mut().push(completed_label_binding);
    }

    pub fn unbind_item(&self) {
        // Unbind
        for binding in self.bindings.borrow().iter() {
            binding.unbind();
        }

        // Clear the vector
        self.bindings.borrow_mut().clear();
    }
}
