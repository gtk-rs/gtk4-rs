use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    // Init `gtk::Builder` from file
    let builder = gtk::Builder::from_string(include_str!("window.ui"));

    // Get window and button from `gtk::Builder`
    let window: ApplicationWindow = builder
        .object("window")
        .expect("Could not get object `window` from builder.");
    let button: Button = builder
        .object("button")
        .expect("Could not get object `button` from builder.");

    // Set application
    window.set_application(Some(app));

    // Connect callback
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Add button
    window.set_child(Some(&button));
    window.present();
}
//ANCHOR_END: build_ui
