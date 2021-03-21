use std::{env::args, thread, time::Duration};

use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindowBuilder, ButtonBuilder};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(on_activate);

    // Get command-line arguments
    let args: Vec<String> = args().collect();
    // Run the application
    app.run(&args);
}

// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .build();

    // Create a button
    let button = ButtonBuilder::new()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // ANCHOR: callback
    // Connect callback
    button.connect_clicked(move |_| {
        // The long running operation runs now in a separate thread
        thread::spawn(move || {
            let ten_seconds = Duration::from_secs(10);
            thread::sleep(ten_seconds);
        });
    });
    // ANCHOR_END: callback

    // Add button
    window.set_child(Some(&button));
    window.present();
}
