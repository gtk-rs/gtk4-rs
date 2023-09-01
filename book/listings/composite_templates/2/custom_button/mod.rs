use glib::Object;
use gtk::glib;

// ANCHOR: mod
mod imp;

glib::wrapper! {
    pub(crate)struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable,
                    gtk::Buildable, gtk::ConstraintTarget;
}
// ANCHOR_END: mod

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
