mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget;
}

impl CustomButton {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
    }

    pub fn with_label(label: &str) -> Self {
        glib::Object::new(&[("label", &label)]).expect("Failed to create Button")
    }
}
