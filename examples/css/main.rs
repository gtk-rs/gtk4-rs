use gtk::prelude::*;

use gtk::gdk::Display;
use gtk::{
    Application, ApplicationWindow, Box as Box_, Button, ComboBoxText, CssProvider, Entry,
    Orientation, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
// Basic CSS: we change background color, we set font color to black and we set it as bold.
const STYLE: &str = "
entry.entry1 {
    background: linear-gradient(to right, #f00, #0f0);
    color: blue;
    font-weight: bold;
}

button {
    /* If we don't put it, the yellow background won't be visible */
    background-image: none;
}

button.button1:hover {
    transition: 500ms;
    color: red;
    background-color: yellow;
}

combobox button.combo box {
    padding: 5px;
}
combobox box arrow {
    -gtk-icon-source: -gtk-icontheme('pan-down-symbolic');
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;
    border-top: 5px solid black;
}";

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

    let combo = ComboBoxText::new();
    combo.append_text("option 1");
    combo.append_text("option 2");
    combo.append_text("option 3");
    combo.set_active(Some(0));

    vbox.append(&button);
    vbox.append(&entry);
    vbox.append(&combo);
    // Then we add the container inside our window.
    window.set_child(Some(&vbox));

    application.connect_activate(move |_| {
        window.show();
    });
}

fn main() {
    let application = Application::new(Some("com.github.css"), Default::default());
    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_data(STYLE.as_bytes());
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        StyleContext::add_provider_for_display(
            &Display::default().expect("Error initializing gtk css provider."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // We build the application UI.
        build_ui(app);
    });
    application.run();
}
