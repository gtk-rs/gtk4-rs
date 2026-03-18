mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomButton {
    pub fn new(label: &str) -> Self {
        glib::Object::builder().property("label", label).build()
    }
}
