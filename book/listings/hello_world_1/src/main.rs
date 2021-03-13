use gtk::prelude::*;
use gtk::Application;

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
        .expect("Initialization failed...");

    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();
    // Run the application
    app.run(&args);
}
