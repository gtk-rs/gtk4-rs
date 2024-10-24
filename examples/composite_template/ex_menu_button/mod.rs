mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct ExMenuButton(ObjectSubclass<imp::ExMenuButton>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
