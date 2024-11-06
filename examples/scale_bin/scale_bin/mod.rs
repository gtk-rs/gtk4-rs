mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct ScaleBin(ObjectSubclass<imp::ScaleBin>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ScaleBin {
    fn default() -> Self {
        glib::Object::new()
    }
}
