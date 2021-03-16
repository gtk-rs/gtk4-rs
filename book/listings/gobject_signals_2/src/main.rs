use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));

    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();
    // Run the application
    app.run(&args);
}

// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindow::new(application);

    // Set the window title
    window.set_title(Some("My GTK App"));

    // Create a button
    let button = Button::with_label("Press me!");

    // Set the button margins
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // ANCHOR: callback
    // Connect callback
    button
        .connect_local("clicked", false, move |args| {
            // Get the button from the arguments
            let button = args.get(0).unwrap().get::<Button>().unwrap().unwrap();
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
