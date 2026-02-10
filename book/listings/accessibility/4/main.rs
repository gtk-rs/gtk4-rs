use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, Label, Orientation, Revealer,
    RevealerTransitionType, accessible, glib,
};

const APP_ID: &str = "org.gtk_rs.Accessibility4";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

// ANCHOR: collapsible
fn build_ui(app: &Application) {
    let container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_start(12)
        .margin_end(12)
        .margin_top(12)
        .margin_bottom(12)
        .build();

    // Create a toggle button that controls visibility
    let toggle_button = Button::builder().label("Details").build();

    // The content that will be shown/hidden
    let revealer = Revealer::builder()
        .transition_type(RevealerTransitionType::SlideDown)
        .reveal_child(false)
        .build();
    let content = Label::new(Some(
        "Here are some additional details that can be expanded.",
    ));
    revealer.set_child(Some(&content));

    // Set initial accessible state
    toggle_button.update_state(&[accessible::State::Expanded(Some(false))]);
    toggle_button
        .update_relation(&[accessible::Relation::Controls(&[revealer.upcast_ref()])]);

    // Toggle visibility when clicked
    let revealer_clone = revealer.clone();
    toggle_button.connect_clicked(move |button| {
        let is_revealed = revealer_clone.reveals_child();
        let new_state = !is_revealed;

        revealer_clone.set_reveal_child(new_state);

        // Update the accessible state to match
        button.update_state(&[accessible::State::Expanded(Some(new_state))]);

        // Update button label to reflect state
        if new_state {
            button.set_label("Details (expanded)");
        } else {
            button.set_label("Details");
        }
    });

    container.append(&toggle_button);
    container.append(&revealer);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Collapsible Section")
        .default_width(350)
        .default_height(200)
        .child(&container)
        .build();

    window.present();
}
// ANCHOR_END: collapsible
