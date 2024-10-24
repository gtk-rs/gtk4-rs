pub mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct CustomTag(ObjectSubclass<imp::CustomTag>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomTag {
    pub fn new(label: &str) -> Self {
        glib::Object::builder()
            .property("label", label)
            .property("has-close-button", true)
            .build()
    }
}
