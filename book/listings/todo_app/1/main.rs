mod todo_object;
mod todo_row;
mod window;

use gdk::Display;
use gtk::prelude::*;
use gtk::Application;
use gtk::{gdk, gio};
use gtk::{CssProvider, SignalListItemFactory, StyleContext};

use todo_object::TodoObject;
use todo_row::TodoRow;
use window::Window;

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_startup(|_| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        StyleContext::add_provider_for_display(
            &Display::default().expect("Error initializing GTK CSS provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });
    app.connect_activate(|app| {
        let window = Window::new(app);
        window.show();
    });

    // Run the application
    app.run();
}
