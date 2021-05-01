use gio::{Settings, SettingsBindFlags};
use gtk::gio;
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindowBuilder, SwitchBuilder};

fn main() {
    // ANCHOR: application
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    // ANCHOR_END: application
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    // Create a window
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .build();

    // Initialize settings
    let settings = Settings::new("org.gtk.example");

    // Create a switch
    let switch = SwitchBuilder::new()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .valign(Align::Center)
        .halign(Align::Center)
        .build();

    // ANCHOR: settings_bind
    settings
        .bind("is-switch-enabled", &switch, "state")
        .flags(SettingsBindFlags::DEFAULT)
        .build();
    // ANCHOR_END: settings_bind

    // Add button
    window.set_child(Some(&switch));
    window.present();
}
