mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct DerivedButton(ObjectSubclass<imp::DerivedButton>)
        @extends gtk::Widget, gtk::Button, crate::base_button::BaseButton,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for DerivedButton {
    fn default() -> Self {
        glib::Object::new()
    }
}
