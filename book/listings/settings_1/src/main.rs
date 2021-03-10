use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindowBuilder, SwitchBuilder};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
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
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .build();

    // ANCHOR: button
    // Create a button
    let button = SwitchBuilder::new()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .valign(Align::Center)
        .halign(Align::Center)
        .build();
    // ANCHOR_END: button

    // Add button
    window.set_child(Some(&button));
    window.present();
}
