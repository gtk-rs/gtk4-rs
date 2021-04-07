use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ButtonBuilder};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(on_activate);

    // Run the application
    app.run();
}

// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindow::new(application);

    // Set the window title
    window.set_title(Some("My GTK App"));

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
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });
    // ANCHOR_END: callback

    // Add button
    window.set_child(Some(&button));
    window.present();
}
