use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

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
    // Create a button
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // ANCHOR: callback
    // Connect callback
    button.connect_local("clicked", false, move |args| {
        // Get the button from the arguments
        let button = args[0]
            .get::<Button>()
            .expect("The value needs to be of type `Button`.");
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
        None
    });
    // ANCHOR_END: callback

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}