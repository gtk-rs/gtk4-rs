mod integer_object;

use gtk::gio;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindowBuilder, Label, ListView, NoSelection, PolicyType,
    ScrolledWindowBuilder, SignalListItemFactory,
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

    // ANCHOR: model
    let model = gio::ListStore::new(IntegerObject::static_type());
    for number in 0..1000 {
        let integer_object = IntegerObject::from_integer(number);
        model.append(&integer_object);
    }
    // ANCHOR_END: model

    // ANCHOR: factory_setup
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let label = Label::new(None);
        list_item.set_child(Some(&label));
    });
    // ANCHOR_END: factory_setup

    // ANCHOR: factory_bind
    factory.connect_bind(move |_, list_item| {
        // Get `IntegerObject` from `ListItem`
        let integer_object = list_item
            .item()
            .expect("The item has to exist.")
            .downcast::<IntegerObject>()
            .expect("The item has to be an `IntegerObject`.");

        // Get `i32` from `IntegerObject`
        let number = integer_object
            .property("number")
            .expect("The property needs to exist and be readable.")
            .get::<i32>()
            .expect("The property needs to be of type `i32`.");

        // Get `Label` from `ListItem`
        let label = list_item
            .child()
            .expect("The child has to exist.")
            .downcast::<Label>()
            .expect("The child has to be a `Label`.");

        // Setting "label" to "number"
        label.set_label(&number.to_string());
    });
    // ANCHOR_END: factory_bind

    // ANCHOR: selection_list
    let selection_model = NoSelection::new(Some(&model));
    let list_view = ListView::new(Some(&selection_model), Some(&factory));
    // ANCHOR_END: selection_list

    // ANCHOR: scrolled_window
    let scrolled_window = ScrolledWindowBuilder::new()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();
    window.set_child(Some(&scrolled_window));
    window.show();
    // ANCHOR_END: scrolled_window
}
