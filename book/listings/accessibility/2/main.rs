use gtk::prelude::*;
use gtk::{
    accessible, glib, Application, ApplicationWindow, Entry, Label, Orientation,
};

const APP_ID: &str = "org.gtk_rs.Accessibility2";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

// ANCHOR: labelled_by
fn build_ui(app: &Application) {
    let container = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .margin_start(12)
        .margin_end(12)
        .margin_top(12)
        .margin_bottom(12)
        .build();

    let label = Label::new(Some("Username:"));
    let entry = Entry::new();

    // Tell assistive technologies that the entry is labelled by this label
    entry.update_relation(&[accessible::Relation::LabelledBy(&[label.upcast_ref()])]);

    container.append(&label);
    container.append(&entry);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Form Field")
        .default_width(300)
        .default_height(100)
        .child(&container)
        .build();

    window.present();
}
// ANCHOR_END: labelled_by
