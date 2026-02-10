mod imp;

use gtk::prelude::*;
use gtk::{Label, glib};

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomButton {
    pub fn new(label: &str) -> Self {
        let obj: Self = glib::Object::builder().build();
        let child = Label::new(Some(label));
        child.set_parent(&obj);
        obj
    }
}
