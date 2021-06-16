mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct VirtualMethodsAppWindow(ObjectSubclass<imp::VirtualMethodsAppWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow;
}

impl VirtualMethodsAppWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create VideoPlayerWindow")
    }
}
