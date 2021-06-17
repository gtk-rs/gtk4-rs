mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;

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
}
