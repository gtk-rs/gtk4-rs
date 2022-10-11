mod custom_window;

use custom_window::Window;
use gtk::prelude::*;
use gtk::{Application, Button};

// ANCHOR: main
const APP_ID: &str = "org.gtk_rs.SavingWindowState1";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
// ANCHOR_END: main

fn build_ui(app: &Application) {
    // Create a window
    let window = Window::new(app);

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

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Add button
    window.set_child(Some(&button));
    window.present();
}
