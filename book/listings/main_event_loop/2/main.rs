use std::{thread, time::Duration};

use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Button};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Create a button
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // ANCHOR: callback
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        // The long running operation runs now in a separate thread
        thread::spawn(move || {
            let five_seconds = Duration::from_secs(5);
            thread::sleep(five_seconds);
        });
    });
    // ANCHOR_END: callback

    // Add button
    window.set_child(Some(&button));
    window.present();
}
