mod imp;

use gtk::{gdk, glib};

glib::wrapper! {
    pub struct ContentProvider(ObjectSubclass<imp::ContentProvider>)
        @extends gdk::ContentProvider;
}

impl Default for ContentProvider {
    fn default() -> Self {
        glib::Object::new()
    }
}
