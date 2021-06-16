mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct DerivedButton(ObjectSubclass<imp::DerivedButton>)
        @extends gtk::Widget, gtk::Button, crate::base_button::BaseButton;
}
