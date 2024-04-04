// TODO - EntryCompletion is deprecated without replacement in GTK 4.10. This
// example should be updated to remove the deprecated code when a replacement
// lands. See: https://gitlab.gnome.org/GNOME/gtk/-/issues/5689
#![allow(deprecated)]

use gtk::{gio, glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.entry-completion")
        .build();
    application.connect_activate(build_ui);

    // When activated, shuts down the application
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(
        glib::clone!(@weak application => move |_action, _parameter| {
            application.quit();
        }),
    );
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
    application.add_action(&quit);

    // Run the application
    application.run()
}

fn build_ui(application: &gtk::Application) {
    // create the main window
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Entry with autocompletion")
        .default_width(600)
        .default_height(480)
        .build();

    // Create a title label
    let win_title = gtk::Label::default();
    win_title.set_markup("<big>Which country would you like to spend a holiday in?</big>");

    // Create an EntryCompletion widget
    let completion_countries = gtk::EntryCompletion::new();
    // Use the first (and only) column available to set the autocompletion text
    completion_countries.set_text_column(0);
    // how many keystrokes to wait before attempting to autocomplete?
    completion_countries.set_minimum_key_length(1);
    // whether the completions should be presented in a popup window
    completion_countries.set_popup_completion(true);

    // Create a ListStore of items
    // These will be the source for the autocompletion
    // as the user types into the field
    // For a more evolved example of ListStore see src/bin/list_store.rs
    let ls = create_list_model();
    completion_countries.set_model(Some(&ls));

    let input_field = gtk::Entry::new();
    input_field.set_completion(Some(&completion_countries));

    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .margin_bottom(24)
        .build();
    row.append(&win_title);
    input_field.set_margin_top(10);
    row.append(&input_field);

    window.set_child(Some(&row));

    // show everything
    window.present();
}

struct Data {
    description: String,
}

fn create_list_model() -> gtk::ListStore {
    let data: [Data; 4] = [
        Data {
            description: "France".to_string(),
        },
        Data {
            description: "Italy".to_string(),
        },
        Data {
            description: "Sweden".to_string(),
        },
        Data {
            description: "Switzerland".to_string(),
        },
    ];
    let store = gtk::ListStore::new(&[glib::Type::STRING]);
    for d in data.iter() {
        store.set(&store.append(), &[(0, &d.description)]);
    }
    store
}
