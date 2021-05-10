mod integer_object;

use gtk::prelude::*;
use gtk::{
    gio, Application, ApplicationWindowBuilder, Label, ListView, NoSelection, PolicyType,
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
        .build();

    let model = gio::ListStore::new(IntegerObject::static_type());
    for number in 0..1000 {
        let integer_object = IntegerObject::from_integer(number);
        model.append(&integer_object);
    }

    let factory = SignalListItemFactory::new();
    // The "setup" stage is used for creating the widgets
    factory.connect_setup(move |_, list_item| {
        let label = Label::new(None);
        list_item.set_child(Some(&label));
    });

    // The bind stage is used for "binding" the data to the created widgets on the "setup" stage
    factory.connect_bind(move |_, list_item| {
        // Get `IntegerObject` from `ListItem`
        let integer_object = list_item
            .item()
            .expect("The item has to exist.")
            .downcast::<IntegerObject>()
            .expect("The item has to be an `IntegerObject`");

        // Get `i32` from `IntegerObject`
        let number = integer_object
            .property("number")
            .expect("The property needs to exist and be readable.")
            .get::<i32>()
            .expect("The property needs to be of type `bool`.");

        // Get `Label` from `ListItem`
        let label = list_item
            .child()
            .expect("The child has to exist.")
            .downcast::<Label>()
            .expect("The child has to be a `Label`");

        // Setting "label" to "number"
        label.set_label(&number.to_string());
    });

    let selection_model = NoSelection::new(Some(&model));

    let list_view = ListView::new(Some(&selection_model), Some(&factory));

    let scrolled_window = ScrolledWindowBuilder::new()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();

    window.set_child(Some(&scrolled_window));
    window.show();
}
