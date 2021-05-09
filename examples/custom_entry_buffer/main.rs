mod entry_buffer;

use entry_buffer::CustomEntryBuffer;
use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.entry-buffer"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Custom Entry Buffer"));
    window.set_default_size(500, 500);

    let container = gtk::Box::new(gtk::Orientation::Vertical, 12);
    container.set_valign(gtk::Align::Center);
    container.set_halign(gtk::Align::Center);

    let buffer = CustomEntryBuffer::new();

    let text1 = gtk::Text::new();
    text1.set_buffer(&buffer);
    container.append(&text1);

    let text2 = gtk::Text::new();
    text2.set_buffer(&buffer);
    container.append(&text2);

    let entry = gtk::Entry::new();
    entry.set_buffer(&buffer);
    container.append(&entry);

    window.set_child(Some(&container));
    window.show();
}
