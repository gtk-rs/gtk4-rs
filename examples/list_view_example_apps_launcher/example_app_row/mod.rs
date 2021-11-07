mod imp;

use crate::string_object::StringObject;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct ExampleAppRow(ObjectSubclass<imp::ExampleAppRow>)
        @extends gtk::Widget, gtk::Box;
}

impl Default for ExampleAppRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ExampleAppRow {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create ExampleAppRow")
    }

    pub fn set_string_object(&self, string_object: StringObject) {
        let self_ = imp::ExampleAppRow::from_instance(self);
        let string = string_object
            .property("string")
            .ok()
            .unwrap()
            .get::<String>()
            .ok()
            .unwrap();

        self_.name.set_text(&string);
    }
}
