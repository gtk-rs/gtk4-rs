mod imp;

use gtk::{gdk, glib};

glib::wrapper! {
    pub struct ContentProvider(ObjectSubclass<imp::ContentProvider>)
        @extends gdk::ContentProvider;
}

impl ContentProvider {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl Default for ContentProvider {
    fn default() -> Self {
        Self::new()
    }
}
