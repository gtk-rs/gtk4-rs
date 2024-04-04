use gio::Settings;
use gtk::prelude::*;
use gtk::{gio, glib, Align, Application, ApplicationWindow, Switch};

const APP_ID: &str = "org.gtk_rs.Settings1";

fn main() -> glib::ExitCode {
    // ANCHOR: application
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    // ANCHOR_END: application

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // ANCHOR: settings
    // Initialize settings
    let settings = Settings::new(APP_ID);
    // ANCHOR_END: settings

    // ANCHOR: switch
    // Get the last switch state from the settings
    let is_switch_enabled = settings.boolean("is-switch-enabled");

    // Create a switch
    let switch = Switch::builder()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .valign(Align::Center)
        .halign(Align::Center)
        .state(is_switch_enabled)
        .build();
    // ANCHOR_END: switch

    // ANCHOR: connect_state_set
    switch.connect_state_set(move |_, is_enabled| {
        // Save changed switch state in the settings
        settings
            .set_boolean("is-switch-enabled", is_enabled)
            .expect("Could not set setting.");
        // Allow to invoke other event handlers
        glib::Propagation::Proceed
    });
    // ANCHOR_END: connect_state_set

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&switch)
        .build();

    // Present window
    window.present();
}
