mod window;

use gdk::Display;
use gtk::gdk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, CssProvider, StyleContext};

use crate::window::Window;

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to signals
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a new window and show it
    let window = Window::new(app);
    window.show();
}
