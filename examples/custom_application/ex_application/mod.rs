mod imp;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct ExApplication(ObjectSubclass<imp::ExApplication>) @extends gio::Application, gtk::Application, @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for ExApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl ExApplication {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", &"org.gtk_rs.application-subclass")
            .property("flags", &gio::ApplicationFlags::empty())
            .build()
    }
}
