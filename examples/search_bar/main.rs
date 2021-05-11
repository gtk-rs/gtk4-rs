use gtk::prelude::*;

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

    let search_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    search_bar.set_child(Some(&search_box));

    let entry = gtk::SearchEntry::new();
    entry.set_hexpand(true);
    search_box.append(&entry);
    search_bar.connect_entry(&entry);

    let menu_button = gtk::MenuButton::new();
    search_box.append(&menu_button);

    window.show();
}
