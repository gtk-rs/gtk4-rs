mod window;

use gtk::prelude::*;
use gtk::Application;

use window::Window;

// ANCHOR: main
fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Set keyboard accelerator to trigger "win.quit".
    app.set_accels_for_action("win.quit", &["<primary>Q"]);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
// ANCHOR_END: main
