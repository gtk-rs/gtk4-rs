use gio::Settings;
use gtk::gio;
use gtk::{glib::signal::Inhibit, prelude::*};
use gtk::{Align, Application, ApplicationWindow, Switch};

fn main() {
    // ANCHOR: application
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    // ANCHOR_END: application
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // ANCHOR: settings
    // Initialize settings
    let settings = Settings::new("org.gtk.example");
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
            .unwrap();
        // Do not inhibit the the default handler
        Inhibit(false)
    });
    // ANCHOR_END: connect_state_set

    // Add button
    window.set_child(Some(&switch));
    window.present();
}
