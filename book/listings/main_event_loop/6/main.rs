use glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, ApplicationWindow, Button};
use std::thread;
use std::time::Duration;

const APP_ID: &str = "org.gtk_rs.MainEventLoop6";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // ANCHOR: callback
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // The main loop executes the asynchronous block
        glib::spawn_future_local(clone!(
            #[weak]
            button,
            async move {
                // Deactivate the button until the operation is done
                button.set_sensitive(false);
                let enable_button = gio::spawn_blocking(move || {
                    let five_seconds = Duration::from_secs(5);
                    thread::sleep(five_seconds);
                    true
                })
                .await
                .expect("Task needs to finish successfully.");
                // Set sensitivity of button to `enable_button`
                button.set_sensitive(enable_button);
            }
        ));
    });
    // ANCHOR_END: callback

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}
