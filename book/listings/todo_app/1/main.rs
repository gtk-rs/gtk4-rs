mod todo_object;

use gtk::gio;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::{Label, ListView, PolicyType, ScrolledWindow, SignalListItemFactory, SingleSelection};
use todo_object::TodoObject;

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .build();

    let model = gio::ListStore::new(TodoObject::static_type());

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        // Create label
        let label = Label::new(None);
        list_item.set_child(Some(&label));
    });

    let selection_model = SingleSelection::new(Some(&model));
    let list_view = ListView::new(Some(&selection_model), Some(&factory));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();
    window.set_child(Some(&scrolled_window));
    window.show();
}
