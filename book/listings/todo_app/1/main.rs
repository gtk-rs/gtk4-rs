mod todo_object;
mod todo_row;

use glib::BindingFlags;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::{gio, glib};
use gtk::{
    CheckButton, ConstantExpression, Label, ListView, PolicyType, PropertyExpression,
    ScrolledWindow, SignalListItemFactory, SingleSelection,
};

use todo_object::TodoObject;
use todo_row::TodoRow;

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
    for i in 0..1000 {
        model.append(&TodoObject::new(i.to_string(), true));
    }

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        // Create todo row
        let todo_row = TodoRow::new();
        list_item.set_child(Some(&todo_row));
    });

    let bindings_map = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));

    factory.connect_bind(glib::clone!(@strong bindings_map => move |_, list_item| {
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

        // Get `completed_button` from `TodoRow`
        let completed_button = todo_row
            .property("completed-button")
            .expect("The property needs to exist and be readable.")
            .get::<CheckButton>()
            .expect("The property needs to be of type `CheckButton`.");

        // Get `content_label` from `TodoRow`
        let content_label = todo_row
            .property("content-label")
            .expect("The property needs to exist and be readable.")
            .get::<Label>()
            .expect("The property needs to be of type `Label`.");

        // Bind
        let binding_completed = todo_object
            .bind_property("completed", &completed_button, "active")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build()
            .unwrap();
        let binding_content = todo_object
            .bind_property("content", &content_label, "label")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build()
            .unwrap();

        let id = list_item.position();
        bindings_map.borrow_mut().insert(id, (binding_completed, binding_content));
    }));

    factory.connect_unbind(glib::clone!(@strong bindings_map => move |_, list_item| {
        let id = list_item.position();
        let (binding_completed,binding_content )= bindings_map.borrow_mut().remove(&id).unwrap();
        binding_completed.unbind();
        binding_content.unbind();
    }));

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
