mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget;
}

impl CustomButton {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `CustomButton`")
    }
}

impl Default for CustomButton {
    fn default() -> Self {
        Self::new()
    }
}
