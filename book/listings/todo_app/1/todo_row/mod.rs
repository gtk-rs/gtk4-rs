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

    pub fn bind_item(&self, item: &TodoObject) {
        let imp = imp::TodoRow::from_instance(self);
        imp.bind_item(item);
    }

    pub fn unbind_item(&self) {
        let imp = imp::TodoRow::from_instance(self);
        imp.unbind_item();
    }
}
