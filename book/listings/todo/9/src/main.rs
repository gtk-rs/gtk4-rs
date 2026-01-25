mod collection_object;
mod config;
mod task_object;
mod utils;
mod window;

use adw::prelude::*;
use gtk::{gio, glib};
use window::Window;

fn main() -> glib::ExitCode {
    // ANCHOR: resource_loading
    // Load resources from installed location
    let res = gio::Resource::load(config::resources_file())
        .expect("Could not load gresource file");
    gio::resources_register(&res);

    // Create a new application
    let app = adw::Application::builder()
        .application_id(config::app_id())
        .build();
    // ANCHOR_END: resource_loading

    // Connect to signals
    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn setup_shortcuts(app: &adw::Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn build_ui(app: &adw::Application) {
    // Create a new custom window and present it
    let window = Window::new(app);
    window.present();
}
