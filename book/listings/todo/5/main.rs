mod task_object;
mod task_row;
mod utils;
mod window;

use gtk::gio;
use gtk::prelude::*;
use window::Window;

static APP_ID: &str = "org.gtk_rs.Todo5";

// ANCHOR: main
fn main() {
    gio::resources_register_include!("todo_5.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    //        👇 changed
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

//                       👇 changed
fn setup_shortcuts(app: &adw::Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

//                👇 changed
fn build_ui(app: &adw::Application) {
    // Create a new custom window and show it
    let window = Window::new(app);
    window.show();
}
// ANCHOR_END: main
