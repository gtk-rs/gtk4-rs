use gio::Settings;
use glib::signal::Inhibit;
use gtk::{gio, glib};
use gtk::{subclass::prelude::*, ApplicationWindow};

// ANCHOR: imp
pub struct CustomWindow {
    pub settings: Settings,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomWindow {
    const NAME: &'static str = "CustomWindow";
    type Type = super::CustomWindow;
    type ParentType = ApplicationWindow;

    fn new() -> Self {
        Self {
            settings: Settings::new("org.gtk.example"),
        }
    }
}
impl ObjectImpl for CustomWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        // Load latest window state
        obj.load_window_size();
    }
}
impl WidgetImpl for CustomWindow {}
impl WindowImpl for CustomWindow {
    // Save window state right before the window will be closed
    fn close_request(&self, obj: &Self::Type) -> Inhibit {
        if let Err(err) = obj.save_window_size() {
            log::error!("Failed to save window state, {}", &err);
        }
        // Do not inhibit the the default handler
        Inhibit(false)
    }
}
impl ApplicationWindowImpl for CustomWindow {}
// ANCHOR_END: imp
