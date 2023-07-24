use gdk::Display;
use gtk::prelude::*;
use gtk::{gdk, glib, Application, ApplicationWindow, Button, CssProvider};

const APP_ID: &str = "org.gtk_rs.Css4";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    // ANCHOR: buttons
    // Create buttons
    let button_1 = Button::with_label("Press me!");
    let button_2 = Button::with_label("Press me!");

    button_1.set_widget_name("button-1");
    // ANCHOR_END: buttons

    // Create `gtk_box` and add buttons
    let gtk_box = gtk::Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .build();
    gtk_box.append(&button_1);
    gtk_box.append(&button_2);

    // Create a new window and show it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();
    window.present();
}
// ANCHOR_END: build_ui
