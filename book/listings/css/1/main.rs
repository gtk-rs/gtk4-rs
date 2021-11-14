use gdk::Display;
use gtk::gdk;
use gtk::prelude::*;
use gtk::{Application, CssProvider, StyleContext, Window};

fn main() {
    // Initialize logger
    pretty_env_logger::init();

    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.Todo")
        .build();

    // Connect to signals
    app.connect_startup(|app| {
        setup_shortcuts(app);
        load_css()
    });
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

fn load_css() {
    // Load the css file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Error initializing GTK CSS provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let scale = gtk::Scale::builder().build();

    // Create a new window and show it
    let window = Window::builder().application(app).child(&scale).build();
    window.show();
}
