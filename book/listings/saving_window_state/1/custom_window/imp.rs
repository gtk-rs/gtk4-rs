use gio::Settings;
use gtk::subclass::prelude::*;
use gtk::{ApplicationWindow, gio, glib};
use std::cell::OnceCell;

// ANCHOR: imp
#[derive(Default)]
pub struct Window {
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = ApplicationWindow;
}
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        // Load latest window state
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}
impl WidgetImpl for Window {}
impl WindowImpl for Window {
    // Save window state right before the window will be closed
    fn close_request(&self) -> glib::Propagation {
        // Save window size
        self.obj()
            .save_window_size()
            .expect("Failed to save window state");
        // Allow to invoke other event handlers
        glib::Propagation::Proceed
    }
}
impl ApplicationWindowImpl for Window {}
// ANCHOR_END: imp
