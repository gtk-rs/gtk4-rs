use gio::Settings;
use glib::signal::Inhibit;
use gtk::{gio, glib};
use gtk::{subclass::prelude::*, ApplicationWindow};

// ANCHOR: imp
pub struct Window {
    pub settings: Settings,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = ApplicationWindow;

    fn new() -> Self {
        Self {
            settings: Settings::new("org.gtk-rs.example"),
        }
    }
}
impl ObjectImpl for Window {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        // Load latest window state
        obj.load_window_size();
    }
}
impl WidgetImpl for Window {}
impl WindowImpl for Window {
    // Save window state right before the window will be closed
    fn close_request(&self, obj: &Self::Type) -> Inhibit {
        if let Err(err) = obj.save_window_size() {
            log::error!("Failed to save window state, {}", &err);
        }
        // Do not inhibit the default handler
        Inhibit(false)
    }
}
impl ApplicationWindowImpl for Window {}
// ANCHOR_END: imp
