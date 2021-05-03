mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget;
}

impl CustomButton {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
    }
}

impl Default for CustomButton {
    fn default() -> Self {
        Self::new()
    }
}
