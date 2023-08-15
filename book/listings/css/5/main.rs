use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.Css5";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // ANCHOR: buttons
    // Create buttons
    let button_1 = Button::with_label("Destructive");
    let button_2 = Button::with_label("Suggested");

    button_1.add_css_class("destructive-action");
    button_2.add_css_class("suggested-action");
    // ANCHOR_END: buttons

    // Create `gtk_box` and add buttons
    let gtk_box = gtk::Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(6)
        .build();
    gtk_box.append(&button_1);
    gtk_box.append(&button_2);

    // Create a new window and present it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();
    window.present();
}
