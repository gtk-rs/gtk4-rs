use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Box, Orientation, Switch};

const APP_ID: &str = "org.gtk_rs.GObjectProperties2";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

// ANCHOR: activate
fn build_ui(app: &Application) {
    // ANCHOR: switch
    // Create the switch
    let switch = Switch::new();

    // Set and then immediately obtain state
    switch.set_property("state", &true);
    let current_state = switch.property::<bool>("state");

    // This prints: "The current state is true"
    println!("The current state is {}", current_state);
    // ANCHOR_END: switch

    // Set up box
    let gtk_box = Box::builder()
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

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}
// ANCHOR_END: activate
