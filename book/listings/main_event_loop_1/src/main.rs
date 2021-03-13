use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindowBuilder, ButtonBuilder};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));

    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();
    // Run the application
    app.run(&args);
}

// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .build();

    // Create a button
    let button = ButtonBuilder::new()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

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
