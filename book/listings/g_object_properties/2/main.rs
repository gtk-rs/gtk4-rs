use gtk::prelude::*;
use gtk::{glib, Align, Application, ApplicationWindow, Box, Orientation, Switch};

const APP_ID: &str = "org.gtk_rs.GObjectProperties3";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

// ANCHOR: activate
fn build_ui(app: &Application) {
    // ANCHOR: switches
    // Create the switches
    let switch_1 = Switch::new();
    let switch_2 = Switch::new();
    // ANCHOR_END: switches

    // ANCHOR: bind_active
    switch_1
        .bind_property("active", &switch_2, "active")
        .bidirectional()
        .build();
    // ANCHOR_END: bind_active

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
