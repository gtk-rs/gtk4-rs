use gtk::glib;
use gtk::prelude::*;

use gtk::gdk::Display;
use gtk::{
    Application, ApplicationWindow, Box as Box_, Button, CssProvider, DropDown, Entry, Orientation,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};

fn main() -> glib::ExitCode {
    let application = Application::new(Some("com.github.css"), Default::default());
    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // We build the application UI.
        build_ui(app);
    });
    application.run()
}

fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.set_title(Some("CSS"));

    // The container container.
    let vbox = Box_::new(Orientation::Vertical, 0);

    let button = Button::with_label("hover me!");
    button.add_css_class("button1");

    let entry = Entry::new();
    entry.add_css_class("entry1");
    entry.set_text("Some text");

    let model = gtk::StringList::new(&["option 1", "option 2", "option 3"]);
    let drop_down = DropDown::new(Some(model), gtk::Expression::NONE);

    vbox.append(&button);
    vbox.append(&entry);
    vbox.append(&drop_down);
    // Then we add the container inside our window.
    window.set_child(Some(&vbox));

    application.connect_activate(move |_| {
        window.present();
    });
}
