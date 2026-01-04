use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, glib};
use std::cell::Cell;

const APP_ID: &str = "org.gtk_rs.GObjectMemoryManagement1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

// ANCHOR: build_ui
fn build_ui(application: &Application) {
    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // A mutable integer
    let number = Cell::new(0);

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    button_increase.connect_clicked(move |_| number.set(number.get() + 1));

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&button_increase)
        .build();

    // Present the window
    window.present();
}
// ANCHOR_END: build_ui
