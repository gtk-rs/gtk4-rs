mod imp;

use gtk::{gdk, glib};

glib::wrapper! {
    pub struct CustomLayoutChild(ObjectSubclass<imp::CustomLayoutChild>)
        @extends gtk::Widget;
}

impl CustomLayoutChild {
    pub fn new(color: gdk::RGBA) -> Self {
        glib::Object::new(&[("color", &color)])
    }
}
