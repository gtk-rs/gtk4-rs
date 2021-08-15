mod todo_object;
mod todo_row;
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
        .application_id("org.gtk.Todo")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a new custom window and show it
    let window = Window::new(app);
    window.show();
}
// ANCHOR_END: main
