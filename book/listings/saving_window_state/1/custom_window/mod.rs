mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Application;
use gtk::{gio, glib};

// ANCHOR: mod
glib::wrapper! {
    pub struct CustomWindow(ObjectSubclass<imp::CustomWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl CustomWindow {
    pub fn new(app: &Application) -> Self {
        let window: Self = Object::new(&[]).expect("Failed to create CustomWindow");
        window.set_application(Some(app));
        window
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        // Get `settings` from `imp::CustomWindow`
        let settings = &imp::CustomWindow::from_instance(self).settings;

        // Get the size of the window
        let size = self.default_size();

        // Get the window state from `settings`
        settings.set_int("window-width", size.0)?;
        settings.set_int("window-height", size.1)?;
        settings.set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        // Get `settings` from `imp::CustomWindow`
        let settings = &imp::CustomWindow::from_instance(self).settings;

        // Set the window state in `settings`
        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        // Set the size of the window
        self.set_default_size(width, height);

        // If the window was maximized when it was closed, maximize it again
        if is_maximized {
            self.maximize();
        }
    }
}
// ANCHOR_END: mod
