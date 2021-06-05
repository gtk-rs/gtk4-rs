mod todo_object;
mod todo_row;

use gtk::gio;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
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

    model.append(&TodoObject::new("What's up?".to_string(), true));

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        // Create todo row
        let todo_row = TodoRow::new();
        list_item.set_child(Some(&todo_row));

        let list_item_expression = ConstantExpression::new(list_item);
        let todo_object_expression = PropertyExpression::new(
            gtk::ListItem::static_type(),
            Some(&list_item_expression),
            "item",
        );

        // Create expression describing `list_item->item->completed`
        let completed_expression = PropertyExpression::new(
            TodoObject::static_type(),
            Some(&todo_object_expression),
            "completed",
        );

        // Create expression describing `list_item->item->content`
        let content_expression = PropertyExpression::new(
            TodoObject::static_type(),
            Some(&todo_object_expression),
            "content",
        );

        // Get widgets from `TodoRow`
        let completed_button = todo_row
            .property("completed-button")
            .expect("The property needs to exist and be readable.")
            .get::<CheckButton>()
            .expect("The property needs to be of type `CheckButton`.");

        let content_label = todo_row
            .property("content-label")
            .expect("The property needs to exist and be readable.")
            .get::<Label>()
            .expect("The property needs to be of type `Label`.");

        // Bind `TodoRow` to `TodoObject`
        completed_expression.bind(&completed_button, "active", Some(&completed_button));
        content_expression.bind(&content_label, "label", Some(&content_label));
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
