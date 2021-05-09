mod integer_object;

use glib::BindingFlags;
use gtk::glib;
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
        let integer_object = list_item
            .item()
            .unwrap()
            .downcast::<IntegerObject>()
            .unwrap();

        let label = list_item.child().unwrap().downcast::<Label>().unwrap();
        integer_object
            .bind_property("number", &label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
    });

    let selection_model = NoSelection::new(Some(&model));

    let list_view = ListView::new(Some(&selection_model), Some(&factory));
    // // Launch the application when an item of the list is activated
    list_view.connect_activate(move |list_view, position| {
        let model = list_view.model().unwrap();
        let integer_object = model
            .item(position)
            .unwrap()
            .downcast::<IntegerObject>()
            .unwrap();

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
