mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct WgpuArea(ObjectSubclass<imp::WgpuArea>)
        @extends gtk::Widget, gtk::GLArea,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for WgpuArea {
    fn default() -> Self {
        glib::Object::new()
    }
}
