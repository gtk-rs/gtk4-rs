use gtk::{glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.entry-undo")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("Entry Undo"));
    window.set_default_size(450, 250);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 12);
    vbox.set_halign(gtk::Align::Center);
    vbox.set_valign(gtk::Align::Center);

    let label = gtk::Label::new(Some(
        "Use Control + Z or Control + Shift + Z to redo changes",
    ));
    vbox.append(&label);

    let entry = gtk::Entry::new();
    entry.set_enable_undo(true);
    vbox.append(&entry);

    window.set_child(Some(&vbox));

    window.present();
}
