mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct CustomOrientable(ObjectSubclass<imp::CustomOrientable>)
        @extends gtk::Widget, @implements gtk::Orientable;
}

impl Default for CustomOrientable {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomOrientable {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
