// ANCHOR: all
// ANCHOR: prelude
use gtk::prelude::*;
// ANCHOR_END: prelude
// ANCHOR: use
use gtk::{glib, Application, ApplicationWindow, Button};
// ANCHOR_END: use
const APP_ID: &str = "org.gtk_rs.HelloWorld3";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    // ANCHOR: button
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // ANCHOR: callback
    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });
    // ANCHOR_END: callback
    // ANCHOR_END: button

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}
// ANCHOR_END: build_ui
// ANCHOR_END: all
