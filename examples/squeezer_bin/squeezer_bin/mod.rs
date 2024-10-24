mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct SqueezerBin(ObjectSubclass<imp::SqueezerBin>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for SqueezerBin {
    fn default() -> Self {
        glib::Object::new()
    }
}
