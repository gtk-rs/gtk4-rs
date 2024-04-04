mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct ScaleBin(ObjectSubclass<imp::ScaleBin>)
        @extends gtk::Widget;
}

impl Default for ScaleBin {
    fn default() -> Self {
        glib::Object::new()
    }
}
