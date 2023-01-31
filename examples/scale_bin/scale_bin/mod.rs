mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct ScaleBin(ObjectSubclass<imp::ScaleBin>) @extends gtk::Widget;
}

impl ScaleBin {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new()
    }
}
