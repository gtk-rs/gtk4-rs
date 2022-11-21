mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct ExButton(ObjectSubclass<imp::ExButton>)
        @extends gtk::Widget,
        @implements gtk::Accessible;
}

impl Default for ExButton {
    fn default() -> Self {
        Self::new()
    }
}

impl ExButton {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}
