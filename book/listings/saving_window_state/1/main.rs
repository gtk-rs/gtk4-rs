mod custom_window;

use custom_window::CustomWindow;
use gtk::prelude::*;
use gtk::{Application, Button};

// ANCHOR: main
fn main() {
    // Initialize logger
    pretty_env_logger::init();

    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
// ANCHOR_END: main

fn build_ui(application: &Application) {
    // Create a window
    let window = CustomWindow::new(application);

    // ANCHOR: button
    // Create a button
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // ANCHOR_END: button

    // Connect callback
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Add button
    window.set_child(Some(&button));
    window.present();
}
