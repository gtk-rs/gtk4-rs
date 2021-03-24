//! # Entry completion example
//!
//! This example demonstrates how to build a list of items and use them
//! to autocomplete a field as the user types in something.

use gtk::glib;
use gtk::prelude::*;

use std::env::args;

use glib::Type;
use gtk::gio::SimpleAction;
use gtk::{
    Application, ApplicationWindow, Box as Box_, Entry, EntryCompletion, Label, ListStore,
    Orientation,
};

struct Data {
    description: String,
}

fn create_list_model() -> ListStore {
    let col_types: [Type; 1] = [Type::STRING];

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
    let store = ListStore::new(&col_types);
    let col_indices: [u32; 1] = [0];
    for d in data.iter() {
        let values: [&dyn ToValue; 1] = [&d.description];
        store.set(&store.append(), &col_indices, &values);
    }
    store
}

fn build_ui(application: &Application) {
    // create the main window
    let window = ApplicationWindow::new(application);
    window.set_title(Some("Entry with autocompletion"));
    window.set_default_size(840, 480);

    // Create a title label
    let win_title = Label::new(None);
    win_title.set_markup("<big>Which country would you like to spend a holiday in?</big>");

    // Create an EntryCompletion widget
    let completion_countries = EntryCompletion::new();
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

    let input_field = Entry::new();
    input_field.set_completion(Some(&completion_countries));

    let row = Box_::new(Orientation::Vertical, 5);
    row.append(&win_title);
    input_field.set_margin_top(10);
    row.append(&input_field);

    window.set_child(Some(&row));

    // show everything
    window.show();
}

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.entry-completion"),
        Default::default(),
    )
    .expect("Initialization failed...");
    application.connect_activate(build_ui);

    // When activated, shuts down the application
    let quit = SimpleAction::new("quit", None);
    quit.connect_activate(
        glib::clone!(@weak application => move |_action, _parameter| {
            application.quit();
        }),
    );
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
    application.add_action(&quit);

    // Run the application
    application.run(&args().collect::<Vec<_>>());
}
