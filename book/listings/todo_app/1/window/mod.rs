mod imp;

use std::fs::File;

use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, CustomFilter, FilterListModel, NoSelection, SignalListItemFactory};

use crate::todo_object::{TodoData, TodoObject};
use crate::todo_row::TodoRow;
use crate::utils::data_path;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        let model = gio::ListStore::new(TodoObject::static_type());
        let window = Object::new(&[("application", app)]).expect("Failed to create Window");

        let imp = imp::Window::from_instance(&window);
        window.set_model(model);
        window.restore_data();

        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            // Create `TodoRow`
            let todo_row = TodoRow::new();
            list_item.set_child(Some(&todo_row));
        });

        factory.connect_bind(move |_, list_item| {
            // Get `TodoObject` from `ListItem`
            let todo_object = list_item
                .item()
                .expect("The item has to exist.")
                .downcast::<TodoObject>()
                .expect("The item has to be an `TodoObject`.");

            // Get `TodoRow` from `ListItem`
            let todo_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TodoRow>()
                .expect("The child has to be a `TodoRow`.");

            todo_row.bind_item(&todo_object);
        });

        factory.connect_unbind(move |_, list_item| {
            // Get `TodoRow` from `ListItem`
            let todo_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TodoRow>()
                .expect("The child has to be a `TodoRow`.");

            todo_row.unbind_item();
        });

        // Set objects
        let filter_model = FilterListModel::new(Some(window.model()), None::<&CustomFilter>);
        let selection_model = NoSelection::new(Some(&filter_model));
        imp.list_view.set_model(Some(&selection_model));
        imp.list_view.set_factory(Some(&factory));

        let model = window.model();
        imp.entry
            .connect_activate(clone!(@weak model => move |entry| {
                let buffer = entry.buffer();
                let content = buffer.text();
                let todo_object = TodoObject::new(content, false);
                model
                    .append(&todo_object);
                buffer.set_text("");
            }));

        imp.clear_button
            .connect_clicked(clone!(@weak model => move |_| {
                let mut position = 0;
                while let Some(item) = model.item(position) {
                    // Get `TodoObject` from `glib::Object`
                    let todo_object = item
                        .downcast_ref::<TodoObject>()
                        .expect("The object needs to be of type `TodoObject`.");

                    if todo_object.completed() {
                        model.remove(position);
                    } else {
                        position += 1;
                    }
                }
            }));

        let builder = gtk::Builder::from_string(include_str!("shortcuts.ui"));
        let shortcuts = builder.object("shortcuts").unwrap();
        window.set_help_overlay(Some(&shortcuts));

        let filter_action = imp.settings.create_action("filter");
        window.add_action(&filter_action);
        app.set_accels_for_action("win.filter::All", &["<primary>a"]);
        app.set_accels_for_action("win.filter::Open", &["<primary>o"]);
        app.set_accels_for_action("win.filter::Done", &["<primary>d"]);
        app.set_accels_for_action("win.show-help-overlay", &["<primary>question"]);

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

    fn model(&self) -> &gio::ListStore {
        // Get state
        let imp = imp::Window::from_instance(&self);

        imp.model.get().expect("Could not get model")
    }

    fn set_model(&self, model: gio::ListStore) {
        let imp = imp::Window::from_instance(self);
        imp.model.set(model).expect("Could not set model");
    }

    fn restore_data(&self) {
        // Deserialize data from file to vector
        let file = File::open(data_path()).expect("Could not open json file.");
        let backup_data: Vec<TodoData> =
            serde_json::from_reader(file).expect("Could not get backup data from json file.");

        for todo_data in backup_data {
            let todo_object = TodoObject::new(todo_data.content, todo_data.completed);
            self.model().append(&todo_object);
        }
    }

    fn set_filter(&self, filter_model: &FilterListModel) {
        // Get state
        let imp = imp::Window::from_instance(&self);

        // Get value from settings
        let value: String = imp.settings.get("filter");

        let filter_all = CustomFilter::new(|_| true);
        let filter_open = CustomFilter::new(|obj: &Object| {
            // Get `TodoObject` from `glib::Object`
            let todo_object = obj
                .downcast_ref::<TodoObject>()
                .expect("The object needs to be of type `TodoObject`.");

            // Only allow open tasks
            !todo_object.completed()
        });
        let filter_done = CustomFilter::new(|obj| {
            // Get `TodoObject` from `glib::Object`
            let todo_object = obj
                .downcast_ref::<TodoObject>()
                .expect("The object needs to be of type `TodoObject`.");

            // Only allow done tasks
            todo_object.completed()
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
