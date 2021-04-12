use gtk::prelude::*;
use gtk::{Align, BoxBuilder, Orientation, Switch};
use gtk::{Application, ApplicationWindowBuilder};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(on_activate);

    // Run the application
    app.run();
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

    // Set and then immediately obtain state
    switch.set_property("state", &true).unwrap();
    let current_state = switch
        .get_property("state")
        .expect("The property needs to exist and be readable.")
        .get_some::<bool>()
        .expect("The property needs to be of type `bool`.");

    // This prints: "The current state is true"
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
