use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Button};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));
    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
}

// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindow::new(application);

    // Create a button
    let button = Button::with_label("Run stuff");

    // Connect callback
    button.connect_clicked(move |_| {
        // GUI is blocked for 10 seconds after the button is pressed
        let ten_seconds = std::time::Duration::from_secs(10);
        std::thread::sleep(ten_seconds);
    });

    // Add button
    window.set_child(Some(&button));
    window.present();
}
