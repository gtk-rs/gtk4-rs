mod imp;

use crate::todo_object::{TodoData, TodoObject};
use crate::todo_row::TodoRow;
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{
    Application, CustomFilter, FilterListModel, NoSelection, SignalListItemFactory,
};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create `Window`.")
    }

    fn model(&self) -> &gio::ListStore {
        // Get state
        self.imp().model.get().expect("Could not get model")
    }

    // ANCHOR: filter
    fn filter(&self) -> Option<CustomFilter> {
        // Get state

        // Get filter_state from settings
        let filter_state: String = self.imp().settings.get("filter");

        // Create custom filters
        let filter_open = CustomFilter::new(|obj| {
            // Get `TodoObject` from `glib::Object`
            let todo_object = obj
                .downcast_ref::<TodoObject>()
                .expect("The object needs to be of type `TodoObject`.");

            // Only allow completed tasks
            !todo_object.is_completed()
        });
        let filter_done = CustomFilter::new(|obj| {
            // Get `TodoObject` from `glib::Object`
            let todo_object = obj
                .downcast_ref::<TodoObject>()
                .expect("The object needs to be of type `TodoObject`.");

            // Only allow done tasks
            todo_object.is_completed()
        });

        // Return the correct filter
        match filter_state.as_str() {
            "All" => None,
            "Open" => Some(filter_open),
            "Done" => Some(filter_done),
            _ => unreachable!(),
        }
    }
    // ANCHOR_END: filter

    // ANCHOR: setup_model
    fn setup_model(&self) {
        // Create new model
        let model = gio::ListStore::new(TodoObject::static_type());

        // Get state and set model
        self.imp().model.set(model).expect("Could not set model");

        // Wrap model with filter and selection and pass it to the list view
        let filter_model =
            FilterListModel::new(Some(self.model()), self.filter().as_ref());
        let selection_model = NoSelection::new(Some(&filter_model));
        self.imp().list_view.set_model(Some(&selection_model));

        // Filter model whenever the value of the key "filter" changes
        self.imp().settings.connect_changed(
            Some("filter"),
            clone!(@weak self as window, @weak filter_model => move |_, _| {
                filter_model.set_filter(window.filter().as_ref());
            }),
        );
    }
    // ANCHOR_END: setup_model

    // ANCHOR: restore_data
    fn restore_data(&self) {
        // Deserialize data from file to vector
        let tasks: Vec<TodoData> = self.imp().settings.get("tasks");

        // Convert `Vec<TodoData>` to `Vec<TodoObject>`
        let todo_objects: Vec<TodoObject> = tasks
            .into_iter()
            .map(|todo_data| TodoObject::new(todo_data.completed, todo_data.content))
            .collect();

        // Insert restored objects into model
        self.model().splice(0, 0, &todo_objects);
    }
    // ANCHOR_END: restore_data

    // ANCHOR: setup_callbacks
    fn setup_callbacks(&self) {
        // Setup callback for activation of the entry
        self.imp()
            .entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.new_task();
            }));

        // Setup callback for clicking (and the releasing) the icon of the entry
        self.imp().entry.connect_icon_release(
            clone!(@weak self as window => move |_,_| {
                window.new_task();
            }),
        );
    }
    // ANCHOR_END: setup_callbacks

    fn new_task(&self) {
        // Get content from entry and clear it
        let buffer = self.imp().entry.buffer();
        let content = buffer.text();
        buffer.set_text("");

        // Add new task to model
        let task = TodoObject::new(false, content);
        self.model().append(&task);
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

            todo_row.bind(&todo_object);
        });

        // Tell factory how to unbind `TodoRow` from `TodoObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `TodoRow` from `ListItem`
            let todo_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TodoRow>()
                .expect("The child has to be a `TodoRow`.");

            todo_row.unbind();
        });

        // Set the factory of the list view
        self.imp().list_view.set_factory(Some(&factory));
    }

    // ANCHOR: setup_actions
    fn setup_actions(&self) {
        // Create action from key "filter" and add to action group "win"
        let action_filter = self.imp().settings.create_action("filter");
        self.add_action(&action_filter);

        // Get model
        let model = self.model();

        // Create action to remove done tasks and add to action group "win"
        let action_remove_done_tasks =
            gio::SimpleAction::new("remove-done-tasks", None);
        action_remove_done_tasks.connect_activate(clone!(@weak model => move |_, _| {
            let mut position = 0;
            while let Some(item) = model.item(position) {
                // Get `TodoObject` from `glib::Object`
                let todo_object = item
                    .downcast_ref::<TodoObject>()
                    .expect("The object needs to be of type `TodoObject`.");

                if todo_object.is_completed() {
                    model.remove(position);
                } else {
                    position += 1;
                }
            }
        }));
        self.add_action(&action_remove_done_tasks);
    }
    // ANCHOR_END: setup_actions
}
