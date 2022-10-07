mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct SquaresWidget(ObjectSubclass<imp::SquaresWidget>) @extends gtk::Widget;
}

impl Default for SquaresWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl SquaresWidget {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}
