mod todo_object;
mod todo_row;
mod utils;
mod window;

use gtk::gio;
use gtk::prelude::*;
use gtk::Application;

use window::Window;

fn main() {
    // Initialize logger
    pretty_env_logger::init();

    gio::resources_register_include!("todo_app_4.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.Todo")
        .build();

    // Connect to signals
    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn build_ui(app: &Application) {
    // Create a new custom window and show it
    let window = Window::new(app);
    window.show();
}
