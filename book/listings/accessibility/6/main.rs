use gtk::prelude::*;
use gtk::{
    accessible, glib, AccessibleInvalidState, Application, ApplicationWindow, Entry, Label,
    Orientation,
};

const APP_ID: &str = "org.gtk_rs.Accessibility6";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn is_valid_email(text: &str) -> bool {
    text.contains('@') && text.contains('.')
}

fn build_ui(app: &Application) {
    // ANCHOR: setup
    let container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_start(12)
        .margin_end(12)
        .margin_top(12)
        .margin_bottom(12)
        .build();

    let label = Label::new(Some("Email:"));
    let entry = Entry::new();
    entry.update_relation(&[accessible::Relation::LabelledBy(&[label.upcast_ref()])]);

    let error_label = Label::builder()
        .label("Please enter a valid email address")
        .css_classes(["error"])
        .visible(false)
        .build();
    // ANCHOR_END: setup

    // ANCHOR: correct
    // Correct: color, text and accessible state indicate the error
    entry.connect_changed({
        let error_label = error_label.clone();
        move |entry| {
            let text = entry.text();
            if !text.is_empty() && !is_valid_email(&text) {
                entry.add_css_class("error");
                error_label.set_visible(true);
                entry.update_state(&[accessible::State::Invalid(
                    AccessibleInvalidState::True,
                )]);
            } else {
                entry.remove_css_class("error");
                error_label.set_visible(false);
                entry.update_state(&[accessible::State::Invalid(
                    AccessibleInvalidState::False,
                )]);
            }
        }
    });
    // ANCHOR_END: correct

    // ANCHOR: window
    container.append(&label);
    container.append(&entry);
    container.append(&error_label);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Form Validation")
        .default_width(350)
        .default_height(200)
        .child(&container)
        .build();

    window.present();
    // ANCHOR_END: window
}
