mod imp;

use crate::todo_object::TodoObject;
use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrList, Attribute};

glib::wrapper! {
    pub struct TodoRow(ObjectSubclass<imp::TodoRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for TodoRow {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoRow {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `TodoRow`.")
    }

    pub fn bind(&self, todo_object: &TodoObject) {
        // Get state
        let imp = imp::TodoRow::from_instance(self);
        let completed_button = imp.completed_button.get();
        let content_label = imp.content_label.get();
        let mut bindings = imp.bindings.borrow_mut();

        // Bind `todo_object.completed` to `todo_row.completed_button.active`
        let completed_button_binding = todo_object
            .bind_property("completed", &completed_button, "active")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build()
            .expect("Could not bind properties");
        // Save binding
        bindings.push(completed_button_binding);

        // Bind `todo_object.content` to `todo_row.content_label.label`
        let content_label_binding = todo_object
            .bind_property("content", &content_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build()
            .expect("Could not bind properties");
        // Save binding
        bindings.push(content_label_binding);

        // Bind `todo_object.completed` to `todo_row.content_label.attributes`
        let content_label_binding = todo_object
            .bind_property("completed", &content_label, "attributes")
            .flags(BindingFlags::SYNC_CREATE)
            .transform_to(|_, active_value| {
                let attribute_list = AttrList::new();
                let active = active_value
                    .get::<bool>()
                    .expect("The value needs to be of type `bool`.");
                if active {
                    // If "active" is true, content of the label will be strikethrough
                    let attribute = Attribute::new_strikethrough(true);
                    attribute_list.insert(attribute);
                }
                Some(attribute_list.to_value())
            })
            .build()
            .expect("Could not bind properties");
        // Save binding
        bindings.push(content_label_binding);
    }

    pub fn unbind(&self) {
        // Get state
        let imp = imp::TodoRow::from_instance(self);

        // Unbind all stored bindings
        for binding in imp.bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
