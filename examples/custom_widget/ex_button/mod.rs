mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct ExButton(ObjectSubclass<imp::ExButton>)
        @extends gtk::Widget,
        @implements gtk::Accessible;
}

impl Default for ExButton {
    fn default() -> Self {
        glib::Object::new()
    }
}
