mod imp;

use crate::todo_object::TodoObject;
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::SignalListItemFactory;
use gtk::{gio, glib};
use gtk::{Application, CustomFilter, FilterListModel};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Window {
    pub fn new(app: &Application, model: gio::ListStore, factory: SignalListItemFactory) -> Self {
        let window = Object::new(&[]).expect("Failed to create Window");

        let imp = imp::Window::from_instance(&window);

        // Set objects
        window.set_application(Some(app));
        let filter_model = FilterListModel::new(Some(&model), None::<&gtk::CustomFilter>);
        let selection_model = gtk::NoSelection::new(Some(&filter_model));
        imp.list_view.set_model(Some(&selection_model));
        imp.list_view.set_factory(Some(&factory));

        imp.entry
            .connect_activate(clone!(@weak model => move |entry| {
                let buffer = entry.buffer();
                let content = buffer.text();
                let todo_object = TodoObject::new(content, false);
                model
                    .append(&todo_object);
                buffer.set_text("");
            }));

        let filter_action = imp.settings.create_action("filter");
        window.add_action(&filter_action);
        app.set_accels_for_action("win.filter::All", &["<primary>a"]);
        app.set_accels_for_action("win.filter::Open", &["<primary>o"]);
        app.set_accels_for_action("win.filter::Done", &["<primary>d"]);

        // Initial filtering
        window.set_filter(&filter_model);

        // Filter whenever the settings key changes
        imp.settings.connect_changed(
            None,
            clone!(@weak window, @weak filter_model => move |_, key| {
                if key == "filter" {
                    window.set_filter(&filter_model);
                }
            }),
        );

        window
    }

    fn set_filter(&self, filter_model: &FilterListModel) {
        let imp = imp::Window::from_instance(&self);

        // Get value from settings
        let value: String = imp.settings.get("filter");

        let filter_all = CustomFilter::new(|_| true);
        let filter_open = CustomFilter::new(|obj: &Object| {
            // Get `TodoObject` from `glib::Object`
            let todo_object = obj
                .downcast_ref::<TodoObject>()
                .expect("The object needs to be of type `TodoObject`.");

            // Get property "number" from `IntegerObject`
            let completed = todo_object
                .property("completed")
                .expect("The property needs to exist and be readable.")
                .get::<bool>()
                .expect("The property needs to be of type `bool`.");

            // Only allow open tasks
            !completed
        });
        let filter_done = CustomFilter::new(|obj| {
            // Get `TodoObject` from `glib::Object`
            let todo_object = obj
                .downcast_ref::<TodoObject>()
                .expect("The object needs to be of type `TodoObject`.");

            // Get property "number" from `IntegerObject`
            let completed = todo_object
                .property("completed")
                .expect("The property needs to exist and be readable.")
                .get::<bool>()
                .expect("The property needs to be of type `bool`.");

            // Only allow done tasks
            completed
        });

        // Set filter model accordingly
        let filter = match value.as_str() {
            "All" => filter_all,
            "Open" => filter_open,
            "Done" => filter_done,
            _ => unimplemented!(),
        };

        // Set filter model
        filter_model.set_filter(Some(&filter));
    }
}
