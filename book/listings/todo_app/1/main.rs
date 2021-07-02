mod todo_object;
mod todo_row;
mod window;

use gdk::Display;
use gtk::gdk;
use gtk::prelude::*;
use gtk::Application;
use gtk::{CssProvider, StyleContext};

use window::Window;

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_startup(load_css);
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn load_css(_app: &Application) {
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
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.show();
}
