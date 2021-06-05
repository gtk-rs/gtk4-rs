mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct TodoRow(ObjectSubclass<imp::TodoRow>)
    @extends gtk::Box, gtk::Widget;
}

impl TodoRow {
    pub fn new() -> Self {
        Object::new(&[]).unwrap()
    }
}
