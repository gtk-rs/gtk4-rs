mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub(crate)struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomButton {
    pub(crate) fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for CustomButton {
    fn default() -> Self {
        Self::new()
    }
}
