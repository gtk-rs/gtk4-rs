use glib::clone;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.MainEventLoop4";

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
    // Create channel that can hold at most 1 message at a time
    let (sender, receiver) = async_channel::bounded(1);
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        glib::spawn_future_local(clone!(
            #[strong]
            sender,
            async move {
                // Deactivate the button until the operation is done
                sender
                    .send(false)
                    .await
                    .expect("The channel needs to be open.");
                glib::timeout_future_seconds(5).await;
                // Activate the button again
                sender
                    .send(true)
                    .await
                    .expect("The channel needs to be open.");
            }
        ));
    });

    // The main loop executes the asynchronous block
    glib::spawn_future_local(clone!(
        #[weak]
        button,
        async move {
            while let Ok(enable_button) = receiver.recv().await {
                button.set_sensitive(enable_button);
            }
        }
    ));
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
