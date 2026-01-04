mod window;

use gtk::prelude::*;
use gtk::{Application, glib};

use crate::window::Window;

const APP_ID: &str = "org.gtk_rs.Css6";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a new window and present it
    let window = Window::new(app);
    window.present();
}
