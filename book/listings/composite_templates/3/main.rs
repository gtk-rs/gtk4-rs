pub mod custom_button;
mod window;

use gtk::prelude::*;
use gtk::{gio, Application};
use window::Window;

const APP_ID: &str = "org.gtk_rs.CompositeTemplates3";

fn main() {
    // Register and include resources
    gio::resources_register_include!("composite_templates_3.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
// ANCHOR_END: build_ui
