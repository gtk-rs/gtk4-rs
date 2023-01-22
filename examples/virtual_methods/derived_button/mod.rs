mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct DerivedButton(ObjectSubclass<imp::DerivedButton>)
        @extends gtk::Widget, gtk::Button, crate::base_button::BaseButton;
}

impl DerivedButton {
    pub fn new() -> Self {
        glib::Object::new_default()
    }
}

impl Default for DerivedButton {
    fn default() -> Self {
        Self::new()
    }
}
