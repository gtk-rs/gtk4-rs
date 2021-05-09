mod integer_object;

use gtk::{gio, Label};
use gtk::{glib::BindingFlags, prelude::*};
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
        .default_height(600)
        .application(app)
        .title("ListView: Applications Launcher")
        .build();

    let model = gio::ListStore::new(IntegerObject::static_type());
    for number in 0..100_000 {
        let integer_object = IntegerObject::from_integer(number);
        model.append(&integer_object);
    }

    let factory = gtk::SignalListItemFactory::new();
    // the "setup" stage is used for creating the widgets
    factory.connect_setup(move |_factory, item| {
        let label = Label::new(None);
        item.set_child(Some(&label));
    });

    // the bind stage is used for "binding" the data to the created widgets on the "setup" stage
    factory.connect_bind(move |_factory, list_item| {
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

    // // A sorter used to sort AppInfo in the model by their name
    // let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
    //     let app_info1 = obj1.downcast_ref::<gio::AppInfo>().unwrap();
    //     let app_info2 = obj2.downcast_ref::<gio::AppInfo>().unwrap();

    //     app_info1
    //         .name()
    //         .to_lowercase()
    //         .cmp(&app_info2.name().to_lowercase())
    //         .into()
    // });
    // let sorted_model = gtk::SortListModel::new(Some(&model), Some(&sorter));
    let selection_model = gtk::SingleSelection::new(Some(&model));

    let list_view = gtk::ListView::new(Some(&selection_model), Some(&factory));
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

    let scrolled_window = gtk::ScrolledWindowBuilder::new()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();

    window.set_child(Some(&scrolled_window));
    window.show();
}
