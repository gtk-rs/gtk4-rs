mod example_app_row;
mod string_object;
use crate::example_app_row::ExampleAppRow;
use crate::string_object::StringObject;
use gtk::gio;
use gtk::prelude::*;
use std::process::Command;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk_rs.examples.example_apps_launcher"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(600)
        .default_height(600)
        .application(app)
        .title("ListView: Applications Launcher")
        .build();

    let examples = [
        "basics",
        "builder_pattern",
        "clipboard", // does not work
        "clock",
        "composite_template",
        "content_provider", // does not work
        "css",
        "custom_application",
        "custom_buildable",
        "custom_editable",
        "custom_layout_manager",
        "custom_orientable",
        "custom_paintable",
        "custom_widget",
        "dialog",
        "entry_completion",
        "entry_undo",
        "flow_box",
        "grid_packing",
        "gtk_builder",
        "list_view_apps_launcher",
        "scale_bin",
        "search_bar",
        "squares",
        "squeezer_bin",
        "text_viewer",
        "video_player",
        "virtual_methods",
    ];

    let model = gio::ListStore::new(StringObject::static_type());
    for i in examples {
        let integer_object = StringObject::new(i.to_string());
        model.append(&integer_object);
    }

    let factory = gtk::SignalListItemFactory::new();
    // the "setup" stage is used for creating the widgets
    factory.connect_setup(move |_factory, item| {
        let row = ExampleAppRow::new();
        item.set_child(Some(&row));
    });

    // the bind stage is used for "binding" the data to the created widgets on the "setup" stage
    factory.connect_bind(move |_factory, list_item| {
        let string_obj = list_item
            .item()
            .unwrap()
            .downcast::<StringObject>()
            .unwrap();

        let child = list_item
            .child()
            .unwrap()
            .downcast::<ExampleAppRow>()
            .unwrap();
        child.set_string_object(string_obj);
    });

    // A sorter used to sort AppInfo in the model by their name
    let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
        let string_obj1 = obj1.downcast_ref::<StringObject>().unwrap();
        let string_obj2 = obj2.downcast_ref::<StringObject>().unwrap();

        let string_prop1 = string_obj1
            .property("string")
            .ok()
            .unwrap()
            .get::<String>()
            .ok()
            .unwrap();
        let string_prop2 = string_obj2
            .property("string")
            .ok()
            .unwrap()
            .get::<String>()
            .ok()
            .unwrap();

        string_prop1
            .to_lowercase()
            .cmp(&string_prop2.to_lowercase())
            .into()
    });
    let sorted_model = gtk::SortListModel::new(Some(&model), Some(&sorter));
    let selection_model = gtk::SingleSelection::new(Some(&sorted_model));

    let list_view = gtk::ListView::new(Some(&selection_model), Some(&factory));
    // Launch the application when an item of the list is activated
    list_view.connect_activate(move |list_view, position| {
        let model = list_view.model().unwrap();
        let string_obj = model
            .item(position)
            .unwrap()
            .downcast::<StringObject>()
            .unwrap();

        let string_prop = string_obj
            .property("string")
            .ok()
            .unwrap()
            .get::<String>()
            .ok()
            .unwrap();

        let output = Command::new(format!("/app/bin/{}", string_prop))
            .output()
            .expect("Failed to execute command");

        println!("{:?}", output.stdout.as_slice().to_ascii_lowercase());
    });

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();

    window.set_child(Some(&scrolled_window));
    window.show();
}
