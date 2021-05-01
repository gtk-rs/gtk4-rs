// ANCHOR: prelude
use gtk::prelude::*;
// ANCHOR_END: prelude
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    // Create a window
    let window = ApplicationWindow::new(application);

    // Set the window title
    window.set_title(Some("My GTK App"));

    // ANCHOR: button
    // Create a button
    let button = Button::with_label("Press me!");

    // Set the button margins
    button.set_margin_top(18);
    button.set_margin_bottom(18);
    button.set_margin_start(18);
    button.set_margin_end(18);

    // Connect callback
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });
    // ANCHOR_END: button
    // Add button
    window.set_child(Some(&button));
    window.present();
}
