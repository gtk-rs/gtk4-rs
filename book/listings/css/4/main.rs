use gdk::Display;
use gtk::gdk;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, ComboBoxText, CssProvider, Entry, Orientation,
    StyleContext,
};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn load_css() {
    // Load the css file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Error initializing GTK CSS provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // The container container.
    let vbox = gtk::Box::new(Orientation::Vertical, 0);

    let button = Button::with_label("hover me!");
    button.add_css_class("button1");

    let entry = Entry::new();
    entry.add_css_class("entry1");
    entry.set_text("Some text");

    let combo = ComboBoxText::new();
    combo.append_text("option 1");
    combo.append_text("option 2");
    combo.append_text("option 3");
    combo.set_active(Some(0));

    vbox.append(&button);
    vbox.append(&entry);
    vbox.append(&combo);

    // Create a new window and show it
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&vbox)
        .build();
    window.show();
}
