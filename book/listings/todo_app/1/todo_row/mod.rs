mod imp;

use crate::todo_object::TodoObject;
use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrList, Attribute};

glib::wrapper! {
    pub struct TodoRow(ObjectSubclass<imp::TodoRow>)
    @extends gtk::Box, gtk::Widget;
}

impl Default for TodoRow {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoRow {
    pub fn new() -> Self {
        let obj: Self = Object::new(&[]).expect("Failed to create `TodoRow`.");
        obj.add_css_class("tile");
        obj
    }

    pub fn bind_item(&self, item: &TodoObject) {
        // Get state
        let imp = imp::TodoRow::from_instance(self);

        // Bind `todo_object.completed` to `todo_row.completed_button.active` and save binding.
        // The binding is bidirectional so changes on both sides will be mirrored by the other side.
        let completed_button_binding = item
            .bind_property("completed", &imp.completed_button.get(), "active")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build()
            .expect("Could not bind properties");
        imp.bindings.borrow_mut().push(completed_button_binding);

        // Bind `todo_object.content` to `todo_row.content_label.label` and save binding.
        // The binding is bidirectional so changes on both sides will be mirrored by the other side.
        let content_label_binding = item
            .bind_property("content", &imp.content_label.get(), "label")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build()
            .expect("Could not bind properties");
        imp.bindings.borrow_mut().push(content_label_binding);

        // Bind `todo_object.completed` to `todo_row.content_label.attributed` and save binding.
        // We transform the boolean `completed` so that whenever completed is true
        // the content of the label will be strikethrough.
        let completed_label_binding = item
            .bind_property("completed", &imp.content_label.get(), "attributes")
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
            .expect("Could not bind properties");
        imp.bindings.borrow_mut().push(completed_label_binding);
    }

    pub fn unbind_item(&self) {
        // Get state
        let imp = imp::TodoRow::from_instance(self);

        // Unbind all stored bindings
        for binding in imp.bindings.borrow().iter() {
            binding.unbind();
        }
        // Clear the vector
        imp.bindings.borrow_mut().clear();
    }
}
