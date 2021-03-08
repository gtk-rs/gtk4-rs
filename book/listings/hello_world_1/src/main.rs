use gtk::prelude::*;
use gtk::Application;

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
        .expect("Initialization failed...");

    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
}
