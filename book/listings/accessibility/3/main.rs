mod custom_button;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};

use custom_button::CustomButton;

const APP_ID: &str = "org.gtk_rs.Accessibility3";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = CustomButton::new("Click me");

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Custom Button")
        .default_width(300)
        .default_height(200)
        .child(&button)
        .build();

    window.present();
}
