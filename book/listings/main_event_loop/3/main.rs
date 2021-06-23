use std::{thread, time::Duration};

use glib::{clone, Continue, MainContext, PRIORITY_DEFAULT};
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Create a button
    let button = Button::builder()
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
        thread::spawn(move || {
            // Deactivate the button until the operation is done
            sender.send(false).unwrap();
            let ten_seconds = Duration::from_secs(10);
            thread::sleep(ten_seconds);
            // Activate the button again
            sender.send(true).unwrap();
        });
    });

    // The main loop executes the closure as soon as it receives the message
    receiver.attach(
        None,
        clone!(@weak button => @default-return Continue(false),
                    move |enable_button| {
                        button.set_sensitive(enable_button);
                        Continue(true)
                    }
        ),
    );
    // ANCHOR_END: callback

    // Add button
    window.set_child(Some(&button));
    window.present();
}
