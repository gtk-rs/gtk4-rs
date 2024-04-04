use gtk::{gdk, glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.css")
        .build();
    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = gtk::CssProvider::new();
        provider.load_from_string(include_str!("style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // We build the application UI.
        build_ui(app);
    });
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("CSS"));

    // The container container.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let button = gtk::Button::with_label("hover me!");
    button.add_css_class("button1");

    let entry = gtk::Entry::new();
    entry.add_css_class("entry1");
    entry.set_text("Some text");

    let model = gtk::StringList::new(&["option 1", "option 2", "option 3"]);
    let drop_down = gtk::DropDown::new(Some(model), gtk::Expression::NONE);

    vbox.append(&button);
    vbox.append(&entry);
    vbox.append(&drop_down);
    // Then we add the container inside our window.
    window.set_child(Some(&vbox));

    application.connect_activate(move |_| {
        window.present();
    });
}
