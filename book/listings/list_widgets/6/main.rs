use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, ConstantExpression, Label, ListView, NoSelection, PolicyType,
    PropertyExpression, ScrolledWindow, SignalListItemFactory, StringList, StringObject,
};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // ANCHOR: string_list
    // Create a `StringList` with number from 0 to 100_000
    // `StringList` implements FromIterator<String>
    let model: StringList = (0..=100_000)
        .into_iter()
        .map(|number| format!("Task number {}", number))
        .collect();
    // ANCHOR_END: string_list

    let factory = SignalListItemFactory::new();
    // ANCHOR: factory_setup
    factory.connect_setup(move |_, list_item| {
        // Create label
        let label = Label::new(None);
        list_item.set_child(Some(&label));

        // Create expression describing `list_item->item->number`
        let list_item_expression = ConstantExpression::new(list_item);
        let string_object_expression = PropertyExpression::new(
            gtk::ListItem::static_type(),
            Some(&list_item_expression),
            "item",
        );
        let string_expression = PropertyExpression::new(
            StringObject::static_type(),
            Some(&string_object_expression),
            "string",
        );

        // Bind "string" to "label"
        string_expression.bind(&label, "label", Some(&label));
    });
    // ANCHOR_END: factory_setup

    let selection_model = NoSelection::new(Some(&model));
    let list_view = ListView::new(Some(&selection_model), Some(&factory));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();

    // Present window
    window.present();
}
