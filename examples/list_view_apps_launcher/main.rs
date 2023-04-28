mod application_row;
use crate::application_row::ApplicationRow;
use gtk::prelude::*;
use gtk::{gio, glib};

fn main() -> glib::ExitCode {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.apps_launcher"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(600)
        .default_height(600)
        .application(app)
        .title("ListView: Applications Launcher")
        .build();

    let model = gio::ListStore::new::<gio::AppInfo>();
    gio::AppInfo::all().iter().for_each(|app_info| {
        model.append(app_info);
    });

    let factory = gtk::SignalListItemFactory::new();
    // the "setup" stage is used for creating the widgets
    factory.connect_setup(move |_factory, item| {
        // In gtk4 < 4.8, you don't need the following line
        // as gtk used to pass GtkListItem directly. In order to make that API
        // generic for potentially future new APIs, it was switched to taking a GObject in 4.8
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = ApplicationRow::new();
        item.set_child(Some(&row));
    });

    // the bind stage is used for "binding" the data to the created widgets on the "setup" stage
    factory.connect_bind(move |_factory, item| {
        // In gtk4 < 4.8, you don't need the following line
        // as gtk used to pass GtkListItem directly. In order to make that API
        // generic for potentially future new APIs, it was switched to taking a GObject in 4.8
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let app_info = item.item().and_downcast::<gio::AppInfo>().unwrap();

        let child = item.child().and_downcast::<ApplicationRow>().unwrap();
        child.set_app_info(&app_info);
    });

    // A sorter used to sort AppInfo in the model by their name
    let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
        let app_info1 = obj1.downcast_ref::<gio::AppInfo>().unwrap();
        let app_info2 = obj2.downcast_ref::<gio::AppInfo>().unwrap();

        app_info1
            .name()
            .to_lowercase()
            .cmp(&app_info2.name().to_lowercase())
            .into()
    });
    let sorted_model = gtk::SortListModel::new(Some(model), Some(sorter));
    let selection_model = gtk::SingleSelection::new(Some(sorted_model));

    let list_view = gtk::ListView::new(Some(selection_model), Some(factory));
    // Launch the application when an item of the list is activated
    list_view.connect_activate(move |list_view, position| {
        let model = list_view.model().unwrap();
        let app_info = model.item(position).and_downcast::<gio::AppInfo>().unwrap();

        let context = list_view.display().app_launch_context();
        if let Err(err) = app_info.launch(&[], Some(&context)) {
            let parent_window = list_view.root().and_downcast::<gtk::Window>().unwrap();

            gtk::AlertDialog::builder()
                .message(format!("Failed to start {}", app_info.name()))
                .detail(err.to_string())
                .modal(true)
                .build()
                .show(Some(&parent_window));
        }
    });

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();

    window.set_child(Some(&scrolled_window));
    window.present();
}
