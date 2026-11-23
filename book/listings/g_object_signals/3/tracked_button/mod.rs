mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct TrackedButton(ObjectSubclass<imp::TrackedButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl TrackedButton {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for TrackedButton {
    fn default() -> Self {
        Self::new()
    }
}
