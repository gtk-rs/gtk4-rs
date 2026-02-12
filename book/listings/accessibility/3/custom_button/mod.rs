mod imp;

use gtk::prelude::*;
use gtk::{Label, accessible, glib};

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
        // Tell assistive technologies that the button is labelled by its child
        obj.update_relation(&[accessible::Relation::LabelledBy(&[child.upcast_ref()])]);
        obj
    }
}
