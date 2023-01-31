mod imp;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct ApplicationRow(ObjectSubclass<imp::ApplicationRow>)
        @extends gtk::Widget, gtk::Box;
}

impl Default for ApplicationRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationRow {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_app_info(&self, app_info: &gio::AppInfo) {
        let imp = self.imp();
        imp.name.set_text(&app_info.name());
        if let Some(desc) = app_info.description() {
            imp.description.set_text(&desc);
        }
        if let Some(icon) = app_info.icon() {
            imp.image.set_from_gicon(&icon);
        }
    }
}
