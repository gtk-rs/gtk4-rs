mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct ExButton(ObjectSubclass<imp::ExButton>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ExButton {
    fn default() -> Self {
        glib::Object::new()
    }
}
