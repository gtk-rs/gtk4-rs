mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct FemtoVGArea(ObjectSubclass<imp::FemtoVGArea>)
        @extends gtk::Widget, gtk::GLArea,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for FemtoVGArea {
    fn default() -> Self {
        glib::Object::new()
    }
}
