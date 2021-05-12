mod integer_object;

use gtk::gio;
use gtk::prelude::*;
use gtk::{
    ConstantExpression, Label, ListView, NoSelection, PolicyType, PropertyExpression,
    ScrolledWindowBuilder, SignalListItemFactory,
};
use integer_object::IntegerObject;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.apps_launcher"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .default_width(600)
        .default_height(300)
        .application(app)
        .title("ListView: Applications Launcher")
        .build();

    let model = gio::ListStore::new(IntegerObject::static_type());
    for number in 0..1000 {
        let integer_object = IntegerObject::from_integer(number);
        model.append(&integer_object);
    }

    let factory = SignalListItemFactory::new();
    // ANCHOR: factory_setup
    factory.connect_setup(move |_, list_item| {
        // Create label
        let label = Label::new(None);
        list_item.set_child(Some(&label));

        // Create expressions describing `list_item->item->number`
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

    let selection_model = NoSelection::new(Some(&model));
    let list_view = ListView::new(Some(&selection_model), Some(&factory));

    list_view.connect_activate(move |list_view, position| {
        let model = list_view.model().unwrap();
        let integer_object = model
            .item(position)
            .expect("The item has to exist.")
            .downcast::<IntegerObject>()
            .expect("The item has to be an `IntegerObject`.");

        let old_number = integer_object
            .property("number")
            .expect("The property needs to exist and be readable.")
            .get::<i32>()
            .expect("The property needs to be of type `i32`.");

        integer_object
            .set_property("number", old_number + 1)
            .unwrap();
    });

    let scrolled_window = ScrolledWindowBuilder::new()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();
    window.set_child(Some(&scrolled_window));
    window.show();
}
