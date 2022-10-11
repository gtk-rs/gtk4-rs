use glib::{clone, timeout_future_seconds, MainContext};
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.MainEventLoop5";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
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
        let main_context = MainContext::default();
        // The main loop executes the asynchronous block
        main_context.spawn_local(clone!(@weak button => async move {
            // Deactivate the button until the operation is done
            button.set_sensitive(false);
            timeout_future_seconds(5).await;
            // Activate the button again
            button.set_sensitive(true);
        }));
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
