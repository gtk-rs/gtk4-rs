use glib::BindingFlags;
use gtk::{glib, Align, Orientation, Switch};
use gtk::{prelude::*, BoxBuilder};
use gtk::{Application, ApplicationWindowBuilder};
fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
        .expect("Initialization failed...");
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

    // ANCHOR: switches
    // Create the switches
    let switch_1 = Switch::new();
    let switch_2 = Switch::new();
    // ANCHOR_END: switches

    // ANCHOR: bind_state
    switch_1
        .bind_property("state", &switch_2, "state")
        .flags(BindingFlags::BIDIRECTIONAL)
        .build();
    // ANCHOR_END: bind_state

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
    gtk_box.append(&switch_1);
    gtk_box.append(&switch_2);
    window.set_child(Some(&gtk_box));
    window.present();
}
// ANCHOR_END: activate
