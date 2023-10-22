use glib::{clone, MainContext, Priority};
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
    let (sender, receiver) = MainContext::channel(Priority::default());
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        RUNTIME.spawn(clone!(@strong sender => async move {
            let response = reqwest::get("https://www.gtk-rs.org").await;
            sender
                .send(response)
                .expect("Could not send through channel");
        }));
    });

    // The main loop executes the closure as soon as it receives the message
    receiver.attach(None, move |response| {
        if let Ok(response) = response {
            println!("Status {}", response.status());
        } else {
            println!("Could not make a `GET` request.");
        }
        glib::ControlFlow::Continue
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
