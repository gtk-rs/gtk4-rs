mod integer_object;

use gtk::{
    gio, Application, ApplicationWindow, Label, ListView, PolicyType, ScrolledWindow,
    SignalListItemFactory, SingleSelection,
};
use gtk::{prelude::*, ListItem};
use integer_object::IntegerObject;

const APP_ID: &str = "org.gtk_rs.ListWidgets2";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // ANCHOR: model
    // Create a `Vec<IntegerObject>` with numbers from 0 to 100_000
    let vector: Vec<IntegerObject> =
        (0..=100_000).into_iter().map(IntegerObject::new).collect();

    // Create new model
    let model = gio::ListStore::new(IntegerObject::static_type());

    // Add the vector to the model
    model.extend_from_slice(&vector);
    // ANCHOR_END: model

    // ANCHOR: factory_setup
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let label = Label::new(None);
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&label));
    });
    // ANCHOR_END: factory_setup

    // ANCHOR: factory_bind
    factory.connect_bind(move |_, list_item| {
        // Get `IntegerObject` from `ListItem`
        let integer_object = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<IntegerObject>()
            .expect("The item has to be an `IntegerObject`.");

        // Get `i32` from `IntegerObject`
        let number = integer_object.property::<i32>("number");

        // Get `Label` from `ListItem`
        let label = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .child()
            .and_downcast::<Label>()
            .expect("The child has to be a `Label`.");

        // Set "label" to "number"
        label.set_label(&number.to_string());
    });
    // ANCHOR_END: factory_bind

    // ANCHOR: selection_list
    let selection_model = SingleSelection::new(Some(model));
    let list_view = ListView::new(Some(selection_model), Some(factory));
    // ANCHOR_END: selection_list

    // ANCHOR: scrolled_window
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
    // ANCHOR_END: scrolled_window
}
