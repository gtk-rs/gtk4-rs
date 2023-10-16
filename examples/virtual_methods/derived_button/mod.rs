mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct DerivedButton(ObjectSubclass<imp::DerivedButton>)
        @extends gtk::Widget, gtk::Button, crate::base_button::BaseButton;
}

impl Default for DerivedButton {
    fn default() -> Self {
        glib::Object::new()
    }
}
