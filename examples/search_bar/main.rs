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
    window.set_default_size(400, 400);
    window.set_title(Some("Search Bar"));

    let header_bar = gtk::HeaderBar::new();
    window.set_titlebar(Some(&header_bar));

    let search_button = gtk::ToggleButton::new();
    search_button.set_icon_name("system-search-symbolic");
    header_bar.pack_end(&search_button);

    let search_bar = gtk::SearchBar::new();
    search_bar.set_valign(gtk::Align::Start);
    window.set_child(Some(&search_bar));

    let search_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
    search_bar.set_child(Some(&search_box));

    let entry = gtk::SearchEntry::new();
    entry.set_hexpand(true);
    search_box.append(&entry);
    search_bar.connect_entry(&entry);

    let label = gtk::Label::new(Some("Type to start search"));
    label.set_hexpand(true);
    search_box.append(&label);

    search_button.connect_toggled(move |_| {
        if search_bar.is_search_mode() {
            search_bar.set_search_mode_enabled(false);
        } else {
            search_bar.set_search_mode_enabled(true);
        };
    });

    entry.connect_search_changed(move |entry| {
        label.set_text(&entry.text());
    });

    window.show();
}
