use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, ListBox, PolicyType, ScrolledWindow};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .build();

    // ANCHOR: list_box
    // Create a `ListBox` and add labels with integers from 0 to 100
    let list_box = ListBox::new();
    for number in 0..=100 {
        let label = Label::new(Some(&number.to_string()));
        list_box.append(&label);
    }
    // ANCHOR_END: list_box

    // ANCHOR: scrolled_window
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_box)
        .build();
    window.set_child(Some(&scrolled_window));
    window.show();
    // ANCHOR_END: scrolled_window
}
