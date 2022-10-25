use glib::Object;
use gtk::glib;

// ANCHOR: mod
mod imp;

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable,
                    gtk::Buildable, gtk::ConstraintTarget;
}
// ANCHOR_END: mod

impl CustomButton {
    pub fn new() -> Self {
        Object::new(&[])
    }
}

impl Default for CustomButton {
    fn default() -> Self {
        Self::new()
    }
}
