mod imp;

use gtk::{gdk, glib};

glib::wrapper! {
    pub struct CustomLayoutChild(ObjectSubclass<imp::CustomLayoutChild>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomLayoutChild {
    pub fn new(color: gdk::RGBA) -> Self {
        glib::Object::builder().property("color", color).build()
    }
}
