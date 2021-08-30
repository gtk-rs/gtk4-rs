mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Application;
use gtk::{gio, glib};

// ANCHOR: mod
glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create `Window`.")
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        // Get `settings` from `imp::Window`
        let settings = &imp::Window::from_instance(self).settings;

        // Get the size of the window
        let size = self.default_size();

        // Set the window state in `settings`
        settings.set_int("window-width", size.0)?;
        settings.set_int("window-height", size.1)?;
        settings.set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        // Get `settings` from `imp::Window`
        let settings = &imp::Window::from_instance(self).settings;

        // Get the window state from `settings`
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
