use gio::{Settings, SettingsBindFlags};
use gtk::prelude::*;
use gtk::{gio, Align, Application, ApplicationWindow, Switch};

const APP_ID: &str = "org.gtk_rs.Settings2";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Initialize settings
    let settings = Settings::new(APP_ID);

    // Create a switch
    let switch = Switch::builder()
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

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&switch)
        .build();

    // Present window
    window.present();
}
