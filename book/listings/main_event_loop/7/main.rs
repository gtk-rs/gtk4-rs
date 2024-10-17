use glib::clone;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.MainEventLoop7";

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
            async move { fetch_user_information(button).await }
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

#[cfg(target_os = "linux")]
async fn fetch_user_information(button: Button) {
    use ashpd::desktop::account::UserInformation;
    use ashpd::WindowIdentifier;

    // Get native of button for window identifier
    let native = button.native().expect("Need to be able to get native.");
    // Get window identifier so that the dialog will be modal to the main window
    let identifier = WindowIdentifier::from_native(&native).await;
    let request = UserInformation::request()
        .reason("App would like to access user information.")
        .identifier(identifier)
        .send()
        .await;

    if let Ok(response) = request.and_then(|r| r.response()) {
        println!("User name: {}", response.name());
    } else {
        println!("Could not access user information.")
    }
}

#[cfg(not(target_os = "linux"))]
async fn fetch_user_information(_button: Button) {
    println!("fetching user information not available outside target_os = \"linux\"");
}
