mod imp;

use crate::custom_tag::CustomTag;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct CustomEditable(ObjectSubclass<imp::CustomEditable>) @extends gtk::Widget, @implements gtk::Editable;
}

impl Default for CustomEditable {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomEditable {
    pub fn new() -> Self {
        glib::Object::new_default()
    }

    pub fn add_tag(&self, tag: &CustomTag) {
        tag.set_parent(self);
    }

    pub fn remove_tag(&self, tag: &CustomTag) {
        tag.unparent();
    }

    pub fn set_show_spinner(&self, show_spinner: bool) {
        let imp = self.imp();
        if imp.show_spinner.get() == show_spinner {
            return;
        }

        if show_spinner {
            let spinner = gtk::Spinner::builder()
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .build();
            spinner.start();

            spinner.set_parent(self);
            imp.spinner.replace(Some(spinner));
        } else if let Some(spinner) = imp.spinner.borrow_mut().take() {
            spinner.unparent();
        }
        imp.show_spinner.set(show_spinner);
    }
}
