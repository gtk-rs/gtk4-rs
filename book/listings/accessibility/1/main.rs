use gtk::prelude::*;
use gtk::{
    AccessibleRole, Application, ApplicationWindow, Box, GestureClick, Label, glib,
};

const APP_ID: &str = "org.gtk_rs.Accessibility1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

// ANCHOR: custom_button
fn build_ui(app: &Application) {
    // Create a custom "button" from a Box
    let custom_button = Box::builder()
        .css_classes(["card", "activatable"])
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        // Set the accessible role to Button
        .accessible_role(AccessibleRole::Button)
        .build();

    let label = Label::new(Some("Click me"));
    custom_button.append(&label);

    // Add click handling
    let gesture = GestureClick::new();
    gesture.connect_released(|_, _, _, _| {
        println!("Custom button clicked!");
    });
    custom_button.add_controller(gesture);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Custom Button")
        .default_width(300)
        .default_height(200)
        .child(&custom_button)
        .build();

    window.present();
}
// ANCHOR_END: custom_button
