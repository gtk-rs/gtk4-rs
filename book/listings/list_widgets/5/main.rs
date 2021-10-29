mod integer_object;

use gtk::gio;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, ConstantExpression, CustomSorter, FilterChange, Label,
    ListView, PolicyType, PropertyExpression, ScrolledWindow, SignalListItemFactory,
    SingleSelection, SortListModel, SorterChange,
};
use integer_object::IntegerObject;

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
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .build();

    // Create a `Vec<IntegerObject>` with numbers from 0 to 100_000
    let vector: Vec<IntegerObject> = (0..=100_000).into_iter().map(IntegerObject::new).collect();

    // Create new model
    let model = gio::ListStore::new(IntegerObject::static_type());

    // Add the vector to the model at position 0 and 0 removals
    model.splice(0, 0, &vector);

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

    // ANCHOR: filter
    let filter = gtk::CustomFilter::new(move |obj| {
        // Get `IntegerObject` from `glib::Object`
        let integer_object = obj
            .downcast_ref::<IntegerObject>()
            .expect("The object needs to be of type `IntegerObject`.");

        // Get property "number" from `IntegerObject`
        let number = integer_object
            .property("number")
            .expect("The property needs to exist and be readable.")
            .get::<i32>()
            .expect("The property needs to be of type `i32`.");

        // Only allow even numbers
        number % 2 == 0
    });
    let filter_model = gtk::FilterListModel::new(Some(&model), Some(&filter));
    // ANCHOR_END: filter

    // ANCHOR: sorter
    let sorter = CustomSorter::new(move |obj1, obj2| {
        // Get `IntegerObject` from `glib::Object`
        let integer_object_1 = obj1
            .downcast_ref::<IntegerObject>()
            .expect("The object needs to be of type `IntegerObject`.");
        let integer_object_2 = obj2
            .downcast_ref::<IntegerObject>()
            .expect("The object needs to be of type `IntegerObject`.");

        // Get property "number" from `IntegerObject`
        let number_1 = integer_object_1
            .property("number")
            .expect("The property needs to exist and be readable.")
            .get::<i32>()
            .expect("The property needs to be of type `i32`.");
        let number_2 = integer_object_2
            .property("number")
            .expect("The property needs to exist and be readable.")
            .get::<i32>()
            .expect("The property needs to be of type `i32`.");

        // Reverse sorting order -> large numbers come first
        number_2.cmp(&number_1).into()
    });
    let sort_model = SortListModel::new(Some(&filter_model), Some(&sorter));
    // ANCHOR_END: sorter

    let selection_model = SingleSelection::new(Some(&sort_model));
    let list_view = ListView::new(Some(&selection_model), Some(&factory));

    // ANCHOR: activate
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

        // Notify that the filter and sorter have been changed
        filter.changed(FilterChange::Different);
        sorter.changed(SorterChange::Different);
    });
    // ANCHOR_END: activate

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();
    window.set_child(Some(&scrolled_window));
    window.show();
}
