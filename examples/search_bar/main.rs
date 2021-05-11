use gtk::prelude::*;
use gtk::glib::clone;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.search_bar"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Search Bar"));
    window.set_default_size(400, 400);

    let search_bar = gtk::SearchBar::new();
    search_bar.set_valign(gtk::Align::Start);
    window.set_child(Some(&search_bar));
    search_bar.set_key_capture_widget(Some(&window));

    let search_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
    search_bar.set_child(Some(&search_box));

    let entry = gtk::SearchEntry::new();
    entry.set_hexpand(true);
    search_box.append(&entry);
    search_bar.connect_entry(&entry);

    let label = gtk::Label::new(None);
    label.set_hexpand(true);
    search_box.append(&label);

    entry.connect_search_changed(clone!(@strong entry => move |_| {
        label.set_text(&entry.text());
    }));

    window.show();
}
