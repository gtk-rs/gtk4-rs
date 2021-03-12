use glib::{clone, MainContext, PRIORITY_DEFAULT};
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindowBuilder, ButtonBuilder};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
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
    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
    // Connect callback
    button.connect_clicked(move |_| {
        let sender = sender.clone();
        // The long running operation runs now in a separate thread
        std::thread::spawn(move || {
            // Deactivate the button until the operation is done
            sender.send(false);
            let ten_seconds = std::time::Duration::from_secs(10);
            std::thread::sleep(ten_seconds);
            // Activate the button again
            sender.send(true);
        });
    });

    // The main loop executes the closure as soon as it receives the message
    receiver.attach(
        None,
        clone!(@weak button => move |enable_button|{
            button.set_sensitive(enable_button);
            glib::Continue(true)
    } ),
    );
    // ANCHOR_END: callback

    // Add button
    window.set_child(Some(&button));
    window.present();
}
