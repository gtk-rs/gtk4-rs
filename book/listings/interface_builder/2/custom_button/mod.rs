mod imp;

use glib::Object;
use gtk::glib;

// ANCHOR: glib_wrapper
glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}
// ANCHOR_END: glib_wrapper

impl CustomButton {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `CustomButton`.")
    }
}

impl Default for CustomButton {
    fn default() -> Self {
        Self::new()
    }
}
