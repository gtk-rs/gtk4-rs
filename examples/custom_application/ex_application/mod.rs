mod imp;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct ExApplication(ObjectSubclass<imp::ExApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for ExApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", "org.gtk_rs.application-subclass")
            .build()
    }
}
