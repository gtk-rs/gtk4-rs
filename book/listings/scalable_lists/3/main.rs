mod integer_object;

use gtk::gio;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindowBuilder, ConstantExpression, Label, ListView, PolicyType,
    PropertyExpression, ScrolledWindowBuilder, SignalListItemFactory, SingleSelection,
};
use integer_object::IntegerObject;

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    // Create a window
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .build();

    let model = gio::ListStore::new(IntegerObject::static_type());
    for number in 0..=1000 {
        let integer_object = IntegerObject::from(number);
        model.append(&integer_object);
    }

    let factory = SignalListItemFactory::new();
    // ANCHOR: factory_setup
    factory.connect_setup(move |_, list_item| {
        // Create label
        let label = Label::new(None);
        list_item.set_child(Some(&label));

        // Create expression describing `list_item->item->number`
        let list_item_expression = ConstantExpression::new(list_item);
        let integer_object_expression = PropertyExpression::new(
            gtk::ListItem::static_type(),
            Some(&list_item_expression),
            "item",
        );
        let number_expression = PropertyExpression::new(
            IntegerObject::static_type(),
            Some(&integer_object_expression),
            "number",
        );

        // Bind "number" to "label"
        number_expression.bind(&label, "label", Some(&label));
    });
    // ANCHOR_END: factory_setup

    let selection_model = SingleSelection::new(Some(&model));
    let list_view = ListView::new(Some(&selection_model), Some(&factory));

    list_view.connect_activate(move |list_view, position| {
        // Get `IntegerObject` from model
        let model = list_view.model().expect("The model has to exist.");
        let integer_object = model
            .item(position)
            .expect("The item has to exist.")
            .downcast::<IntegerObject>()
            .expect("The item has to be an `IntegerObject`.");

        // Increase "number" of `IntegerObject`
        integer_object.increase_number();
    });

    let scrolled_window = ScrolledWindowBuilder::new()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();
    window.set_child(Some(&scrolled_window));
    window.show();
}
