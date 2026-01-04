mod integer_object;

use gtk::{
    Application, ApplicationWindow, Label, ListView, PolicyType, ScrolledWindow,
    SignalListItemFactory, SingleSelection, Widget, gio, glib,
};
use gtk::{ListItem, prelude::*};
use integer_object::IntegerObject;

const APP_ID: &str = "org.gtk_rs.ListWidgets4";

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

    let selection_model = SingleSelection::new(Some(model));
    let list_view = ListView::new(Some(selection_model), Some(factory));

    list_view.connect_activate(move |list_view, position| {
        // Get `IntegerObject` from model
        let model = list_view.model().expect("The model has to exist.");
        let integer_object = model
            .item(position)
            .and_downcast::<IntegerObject>()
            .expect("The item has to be an `IntegerObject`.");

        // Increase "number" of `IntegerObject`
        integer_object.increase_number();
    });

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
