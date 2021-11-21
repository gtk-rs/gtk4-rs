use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to signals
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create buttons
    let button_1 = Button::builder().label("Destructive").build();
    let button_2 = Button::builder().label("Suggested").build();

    button_1.add_css_class("destructive-action");
    button_2.add_css_class("suggested-action");

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

    // Create a new window and show it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();
    window.show();
}
