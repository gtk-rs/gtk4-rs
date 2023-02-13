use gtk::{prelude::*, ListItem};
use gtk::{
    Application, ApplicationWindow, Label, ListView, NoSelection, PolicyType,
    ScrolledWindow, SignalListItemFactory, StringList, StringObject, Widget,
};

const APP_ID: &str = "org.gtk_rs.ListWidgets6";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // ANCHOR: string_list
    // Create a `StringList` with number from 0 to 100_000
    // `StringList` implements FromIterator<String>
    let model: StringList = (0..=100_000)
        .into_iter()
        .map(|number| number.to_string())
        .collect();
    // ANCHOR_END: string_list

    let factory = SignalListItemFactory::new();
    // ANCHOR: factory_setup
    factory.connect_setup(move |_, list_item| {
        // Create label
        let label = Label::new(None);
        let list_item = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem");
        list_item.set_child(Some(&label));

        // Bind `list_item->item->string` to `label->label`
        list_item
            .property_expression("item")
            .chain_property::<StringObject>("string")
            .bind(&label, "label", Widget::NONE);
    });
    // ANCHOR_END: factory_setup

    let selection_model = NoSelection::new(Some(model));
    let list_view = ListView::new(Some(selection_model), Some(factory));

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
