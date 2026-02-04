use gtk::prelude::*;
use gtk::{
    AccessibleRole, Application, ApplicationWindow, Box, GestureClick, Label, glib,
};

const APP_ID: &str = "org.gtk_rs.Accessibility5";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

// ANCHOR: focusable
fn build_ui(app: &Application) {
    // A custom interactive widget that needs to be focusable
    let custom_widget = Box::builder()
        .css_classes(["card", "activatable"])
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .accessible_role(AccessibleRole::Button)
        // Make the widget focusable so keyboard users can reach it
        .focusable(true)
        // Also allow focusing by clicking
        .focus_on_click(true)
        .build();

    let label = Label::new(Some("Focusable custom widget"));
    custom_widget.append(&label);

    let gesture = GestureClick::new();
    gesture.connect_released(|_, _, _, _| {
        println!("Widget activated!");
    });
    custom_widget.add_controller(gesture);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Focusable Widget")
        .default_width(300)
        .default_height(200)
        .child(&custom_widget)
        .build();

    window.present();
}
// ANCHOR_END: focusable
