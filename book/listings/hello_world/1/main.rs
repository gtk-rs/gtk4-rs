use gtk::prelude::*;
use gtk::Application;

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());

    // Run the application
    app.run();
}
