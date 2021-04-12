use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ButtonBuilder};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
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
    button
        .connect_local("clicked", false, move |args| {
            // Get the button from the arguments
            let button = args
                .get(0)
                .expect("There needs to be a first argument.")
                .get::<Button>()
                .expect("The value needs to be of type `Button`.")
                .expect("The value needs to be `Some`.");
            // Set the label to "Hello World!" after the button has been clicked on
            button.set_label("Hello World!");
            None
        })
        .unwrap();
    // ANCHOR_END: callback

    // Add button
    window.set_child(Some(&button));
    window.present();
}
