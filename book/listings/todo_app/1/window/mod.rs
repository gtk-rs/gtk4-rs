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
        let window: Window = Object::new(&[("application", app)]).expect("Failed to create Window");

        window.setup_model();
        window.restore_data();
        window.setup_factory();
        window.setup_callbacks();
        window.set_shortcut_window();
        window.setup_filter_action();
        setup_shortcuts(app);

        window
    }

    fn model(&self) -> &gio::ListStore {
        // Get state
        let imp = imp::Window::from_instance(&self);
        imp.model.get().expect("Could not get model")
    }

    fn setup_model(&self) {
        // Create new model
        let model = gio::ListStore::new(TodoObject::static_type());

        // Get state and set model
        let imp = imp::Window::from_instance(self);
        imp.model.set(model).expect("Could not set model");

        // Wrap model with filter and selection and pass it to the list view
        let filter_model = FilterListModel::new(Some(self.model()), self.filter().as_ref());
        let selection_model = NoSelection::new(Some(&filter_model));
        imp.list_view.set_model(Some(&selection_model));

        // Filter model whenever the value of the key "filter" changes
        imp.settings.connect_changed(
            None,
            clone!(@weak self as window, @weak filter_model => move |_, key| {
                if key == "filter" {
                    filter_model.set_filter(window.filter().as_ref());
                }
            }),
        );
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

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `TodoRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `TodoRow`
            let todo_row = TodoRow::new();
            list_item.set_child(Some(&todo_row));
        });

        // Tell factory how to bind `TodoRow` to a `TodoObject`
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

        // Tell factory how to unbind `TodoRow` from `TodoObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `TodoRow` from `ListItem`
            let todo_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TodoRow>()
                .expect("The child has to be a `TodoRow`.");

            todo_row.unbind_item();
        });

        // Set the factory of the list view
        let imp = imp::Window::from_instance(&self);
        imp.list_view.set_factory(Some(&factory));
    }

    fn setup_callbacks(&self) {
        // Get state
        let imp = imp::Window::from_instance(&self);
        let model = self.model();

        // Setup callback so that activation of the entry
        // creates a new todo object and clears the entry
        imp.entry
            .connect_activate(clone!(@weak model => move |entry| {
                let buffer = entry.buffer();
                let content = buffer.text();
                let todo_object = TodoObject::new(content, false);
                model
                    .append(&todo_object);
                buffer.set_text("");
            }));

        // Setup callback so that click on the clear_button
        // removes all done tasks
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
    }

    fn set_shortcut_window(&self) {
        let builder = gtk::Builder::from_string(include_str!("shortcuts.ui"));
        let shortcuts = builder.object("shortcuts").unwrap();
        self.set_help_overlay(Some(&shortcuts));
    }

    fn setup_filter_action(&self) {
        // Get state
        let imp = imp::Window::from_instance(&self);

        // Create action from key "filter"
        let filter_action = imp.settings.create_action("filter");

        // Add action "filter" to action group "win"
        self.add_action(&filter_action);
    }

    fn filter(&self) -> Option<CustomFilter> {
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

        // Return correct filter
        match value.as_str() {
            "All" => Some(filter_all),
            "Open" => Some(filter_open),
            "Done" => Some(filter_done),
            _ => None,
        }
    }
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter::All", &["<primary>a"]);
    app.set_accels_for_action("win.filter::Open", &["<primary>o"]);
    app.set_accels_for_action("win.filter::Done", &["<primary>d"]);
    app.set_accels_for_action("win.show-help-overlay", &["<primary>question"]);
}
