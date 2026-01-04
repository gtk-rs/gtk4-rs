mod integer_object;

use gtk::{
    Application, ApplicationWindow, CustomFilter, CustomSorter, FilterChange,
    FilterListModel, Label, ListView, PolicyType, ScrolledWindow,
    SignalListItemFactory, SingleSelection, SortListModel, SorterChange, Widget, gio,
    glib,
};
use gtk::{ListItem, prelude::*};
use integer_object::IntegerObject;

const APP_ID: &str = "org.gtk_rs.ListWidgets5";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a `Vec<IntegerObject>` with numbers from 0 to 100_000
    let vector: Vec<IntegerObject> = (0..=100_000).map(IntegerObject::new).collect();

    // Create new model
    let model = gio::ListStore::new::<IntegerObject>();

    // Add the vector to the model
    model.extend_from_slice(&vector);

    let factory = SignalListItemFactory::new();
    // ANCHOR: factory_setup
    factory.connect_setup(move |_, list_item| {
        // Create label
        let label = Label::new(None);
        let list_item = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem");
        list_item.set_child(Some(&label));

        // Bind `list_item->item->number` to `label->label`
        list_item
            .property_expression("item")
            .chain_property::<IntegerObject>("number")
            .bind(&label, "label", Widget::NONE);
    });
    // ANCHOR_END: factory_setup

    // ANCHOR: filter
    let filter = CustomFilter::new(move |obj| {
        // Get `IntegerObject` from `glib::Object`
        let integer_object = obj
            .downcast_ref::<IntegerObject>()
            .expect("The object needs to be of type `IntegerObject`.");

        // Only allow even numbers
        integer_object.number() % 2 == 0
    });
    let filter_model = FilterListModel::new(Some(model), Some(filter.clone()));
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
        let number_1 = integer_object_1.number();
        let number_2 = integer_object_2.number();

        // Reverse sorting order -> large numbers come first
        number_2.cmp(&number_1).into()
    });
    let sort_model = SortListModel::new(Some(filter_model), Some(sorter.clone()));
    // ANCHOR_END: sorter

    let selection_model = SingleSelection::new(Some(sort_model));
    let list_view = ListView::new(Some(selection_model), Some(factory));

    // ANCHOR: activate
    list_view.connect_activate(move |list_view, position| {
        // Get `IntegerObject` from model
        let model = list_view.model().expect("The model has to exist.");
        let integer_object = model
            .item(position)
            .and_downcast::<IntegerObject>()
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
