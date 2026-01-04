mod window;

use gtk::prelude::*;
use gtk::{Application, gio, glib};
use window::Window;

const APP_ID: &str = "org.gtk_rs.Actions7";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("actions_7.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
