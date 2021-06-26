mod todo_object;
mod todo_row;
mod window;

use gdk::Display;
use gtk::prelude::*;
use gtk::Application;
use gtk::{gdk, gio};
use gtk::{CssProvider, ListView, NoSelection, SignalListItemFactory, StyleContext};

use todo_object::TodoObject;
use todo_row::TodoRow;
use window::Window;

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_startup(|_| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        StyleContext::add_provider_for_display(
            &Display::default().expect("Error initializing GTK CSS provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let model = gio::ListStore::new(TodoObject::static_type());

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        // Create todo row
        let todo_row = TodoRow::new();
        list_item.set_child(Some(&todo_row));
    });

    factory.connect_bind(move |_, list_item| {
        // Get `TodoObject` from `ListItem`
        let todo_object = list_item
            .item()
            .expect("The item has to exist.")
            .downcast::<TodoObject>()
            .expect("The item has to be an `TodoObject`.");

        // Get `TodoRow` from `ListItem`
        let todo_row = list_item
            .child()
            .expect("The child has to exist.")
            .downcast::<TodoRow>()
            .expect("The child has to be a `TodoRow`.");

        todo_row.bind_item(&todo_object);
    });

    factory.connect_unbind(move |_, list_item| {
        // Get `TodoRow` from `ListItem`
        let todo_row = list_item
            .child()
            .expect("The child has to exist.")
            .downcast::<TodoRow>()
            .expect("The child has to be a `TodoRow`.");

        todo_row.unbind_item();
    });

    let selection_model = NoSelection::new(Some(&model));
    let list_view = ListView::new(Some(&selection_model), Some(&factory));
    let window = Window::new(app, &list_view, model);

    window.show();
}
