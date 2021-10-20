mod todo_object;
mod todo_row;
mod utils;
mod window;

use gtk::prelude::*;
use gtk::Application;

use window::Window;

// ANCHOR: main
fn main() {
    // Initialize logger
    pretty_env_logger::init();

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
    app.set_accels_for_action("win.filter('All')", &["<primary>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<primary>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<primary>d"]);
    app.set_accels_for_action("win.show-help-overlay", &["<primary>question"]);
}
// ANCHOR_END: main

fn build_ui(app: &Application) {
    // Create a new custom window and show it
    let window = Window::new(app);
    window.show();
}
