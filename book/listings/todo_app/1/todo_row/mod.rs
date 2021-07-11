mod bindings;
mod imp;

use crate::todo_object::TodoObject;
use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct TodoRow(ObjectSubclass<imp::TodoRow>)
    @extends gtk::Box, gtk::Widget;
}

impl TodoRow {
    pub fn new() -> Self {
        let obj: Self = Object::new(&[]).unwrap();
        obj.add_css_class("tile");
        obj
    }

    pub fn set_item(&self, item: Option<&TodoObject>) {
        let imp = imp::TodoRow::from_instance(self);
        imp.set_item(item);
    }
}
