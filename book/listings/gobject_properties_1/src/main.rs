use gtk::prelude::*;
use gtk::{Align, BoxBuilder, Orientation, Switch};
use gtk::{Application, ApplicationWindowBuilder};
use std::env::args;

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
// ANCHOR: activate
// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .build();

    // ANCHOR: switch
    // Create the switch
    let switch = Switch::new();

    switch.set_property("state", &true).unwrap();

    let current_state = switch
        .get_property("state")
        .unwrap()
        .get::<bool>()
        .unwrap()
        .unwrap();
    // This current state will be true
    println!("The current state is {}", current_state);
    // ANCHOR_END: switch

    // Set up box
    let gtk_box = BoxBuilder::new()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&switch);
    window.set_child(Some(&gtk_box));
    window.present();
}
// ANCHOR_END: activate
