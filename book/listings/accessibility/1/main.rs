use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Orientation, accessible, glib};

const APP_ID: &str = "org.gtk_rs.Accessibility1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let container = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    // ANCHOR: icon_button
    // Icon-only button needs an accessible label
    let search_button = Button::builder()
        .icon_name("system-search-symbolic")
        .build();
    search_button.update_property(&[accessible::Property::Label("Search")]);
    // ANCHOR_END: icon_button

    // ANCHOR: description
    // Add additional context with a description
    let settings_button = Button::builder()
        .icon_name("emblem-system-symbolic")
        .build();
    settings_button.update_property(&[
        accessible::Property::Label("Settings"),
        accessible::Property::Description("Open application preferences"),
    ]);
    // ANCHOR_END: description

    container.append(&search_button);
    container.append(&settings_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Icon Buttons")
        .default_width(300)
        .default_height(200)
        .child(&container)
        .build();

    window.present();
}
