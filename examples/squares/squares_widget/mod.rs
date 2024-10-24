mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct SquaresWidget(ObjectSubclass<imp::SquaresWidget>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for SquaresWidget {
    fn default() -> Self {
        glib::Object::new()
    }
}
