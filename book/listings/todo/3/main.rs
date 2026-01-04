mod task_object;
mod task_row;
mod utils;
mod window;

use gdk::Display;
use gtk::prelude::*;
use gtk::{Application, CssProvider, gdk, gio, glib};
use window::Window;

const APP_ID: &str = "org.gtk_rs.Todo3";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("todo_3.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // ANCHOR: connect_startup
    // Connect to signals
    app.connect_startup(|app| {
        setup_shortcuts(app);
        load_css()
    });
    // ANCHOR_END: connect_startup
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

// ANCHOR: load_css
fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_resource("/org/gtk_rs/Todo3/style.css");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
// ANCHOR_END: load_css

fn build_ui(app: &Application) {
    // Create a new custom window and present it
    let window = Window::new(app);
    window.present();
}
