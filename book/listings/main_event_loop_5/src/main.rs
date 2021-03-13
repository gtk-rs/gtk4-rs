use glib::{timeout_future_seconds, MainContext};
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindowBuilder, ButtonBuilder};

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

    // ANCHOR: callback
    // Connect callback
    button.connect_clicked(move |button| {
        let button = button.clone();
        let main_context = MainContext::default();
        // The main loop executes the asynchronous block
        main_context.spawn_local(async move {
            // Deactivate the button until the operation is done
            button.set_sensitive(false);
            timeout_future_seconds(10).await;
            // Activate the button again
            button.set_sensitive(true);
        });
    });
    // ANCHOR_END: callback

    // Add button
    window.set_child(Some(&button));
    window.present();
}
