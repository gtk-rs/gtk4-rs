use glib::BindingFlags;
use gtk::{glib, Align, Orientation, Switch};
use gtk::{prelude::*, Box};
use gtk::{Application, ApplicationWindow};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk.example")
        .build();

    // Connect to "activate" signal
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

// ANCHOR: activate
fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
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
    gtk_box.append(&switch_1);
    gtk_box.append(&switch_2);
    window.set_child(Some(&gtk_box));
    window.present();
}
// ANCHOR_END: activate
