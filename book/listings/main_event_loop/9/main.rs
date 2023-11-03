use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

// ANCHOR: tokio_runtime
const APP_ID: &str = "org.gtk_rs.MainEventLoop9";
static RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."));

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
// ANCHOR_END: tokio_runtime

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
    let (sender, receiver) = async_channel::bounded(1);
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        RUNTIME.spawn(clone!(@strong sender => async move {
            let response = reqwest::get("https://www.gtk-rs.org").await;
            sender.send(response).await.expect("The channel needs to be open.");
        }));
    });

    // The main loop executes the asynchronous block
    glib::spawn_future_local(async move {
        while let Ok(response) = receiver.recv().await {
            if let Ok(response) = response {
                println!("Status: {}", response.status());
            } else {
                println!("Could not make a `GET` request.");
            }
        }
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
