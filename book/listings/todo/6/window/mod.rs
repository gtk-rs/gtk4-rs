mod imp;

use std::fs::File;

use adw::subclass::prelude::*;
use adw::{ActionRow, prelude::*};
use gio::Settings;
use glib::{Object, clone};
use gtk::{Align, CheckButton, CustomFilter, FilterListModel, NoSelection, gio, glib};

use crate::APP_ID;
use crate::task_object::{TaskData, TaskObject};
use crate::utils::data_path;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    fn tasks(&self) -> gio::ListStore {
        self.imp()
            .tasks
            .borrow()
            .clone()
            .expect("Could not get current tasks.")
    }

    fn filter(&self) -> Option<CustomFilter> {
        // Get filter state from settings
        let filter_state: String = self.settings().get("filter");

        // Create custom filters
        let filter_open = CustomFilter::new(|obj| {
            // Get `TaskObject` from `glib::Object`
            let task_object = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            // Only allow completed tasks
            !task_object.is_completed()
        });
        let filter_done = CustomFilter::new(|obj| {
            // Get `TaskObject` from `glib::Object`
            let task_object = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            // Only allow done tasks
            task_object.is_completed()
        });

        // Return the correct filter
        match filter_state.as_str() {
            "All" => None,
            "Open" => Some(filter_open),
            "Done" => Some(filter_done),
            _ => unreachable!(),
        }
    }

    fn setup_tasks(&self) {
        // Create new model
        let model = gio::ListStore::new::<TaskObject>();

        // Get state and set model
        self.imp().tasks.replace(Some(model));

        // ANCHOR: bind_model
        // Wrap model with filter and selection and pass it to the list box
        let filter_model = FilterListModel::new(Some(self.tasks()), self.filter());
        let selection_model = NoSelection::new(Some(filter_model.clone()));
        self.imp().tasks_list.bind_model(
            Some(&selection_model),
            clone!(
                #[weak(rename_to = window)]
                self,
                #[upgrade_or_panic]
                move |obj| {
                    let task_object = obj
                        .downcast_ref()
                        .expect("The object should be of type `TaskObject`.");
                    let row = window.create_task_row(task_object);
                    row.upcast()
                }
            ),
        );
        // ANCHOR_END: bind_model

        // Filter model whenever the value of the key "filter" changes
        self.settings().connect_changed(
            Some("filter"),
            clone!(
                #[weak(rename_to = window)]
                self,
                #[weak]
                filter_model,
                move |_, _| {
                    filter_model.set_filter(window.filter().as_ref());
                }
            ),
        );

        // ANCHOR: connect_items_changed
        // Assure that the task list is only visible when it is supposed to
        self.set_task_list_visible(&self.tasks());
        self.tasks().connect_items_changed(clone!(
            #[weak(rename_to = window)]
            self,
            move |tasks, _, _, _| {
                window.set_task_list_visible(tasks);
            }
        ));
        // ANCHOR_END: connect_items_changed
    }

    // ANCHOR: set_task_list_visible
    /// Assure that `tasks_list` is only visible
    /// if the number of tasks is greater than 0
    fn set_task_list_visible(&self, tasks: &gio::ListStore) {
        self.imp().tasks_list.set_visible(tasks.n_items() > 0);
    }
    // ANCHOR_END: set_task_list_visible

    fn restore_data(&self) {
        if let Ok(file) = File::open(data_path()) {
            // Deserialize data from file to vector
            let backup_data: Vec<TaskData> = serde_json::from_reader(file).expect(
                "It should be possible to read `backup_data` from the json file.",
            );

            // Convert `Vec<TaskData>` to `Vec<TaskObject>`
            let task_objects: Vec<TaskObject> = backup_data
                .into_iter()
                .map(TaskObject::from_task_data)
                .collect();

            // Insert restored objects into model
            self.tasks().extend_from_slice(&task_objects);
        }
    }

    // ANCHOR: create_task_row
    fn create_task_row(&self, task_object: &TaskObject) -> ActionRow {
        // Create check button
        let check_button = CheckButton::builder()
            .valign(Align::Center)
            .can_focus(false)
            .build();

        // Create row
        let row = ActionRow::builder()
            .activatable_widget(&check_button)
            .build();
        row.add_prefix(&check_button);

        // Bind properties
        task_object
            .bind_property("completed", &check_button, "active")
            .bidirectional()
            .sync_create()
            .build();
        task_object
            .bind_property("content", &row, "title")
            .sync_create()
            .build();

        // Return row
        row
    }
    // ANCHOR_END: create_task_row

    fn setup_callbacks(&self) {
        // Setup callback for activation of the entry
        self.imp().entry.connect_activate(clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.new_task();
            }
        ));

        // Setup callback for clicking (and the releasing) the icon of the entry
        self.imp().entry.connect_icon_release(clone!(
            #[weak(rename_to = window)]
            self,
            move |_, _| {
                window.new_task();
            }
        ));
    }

    fn new_task(&self) {
        // Get content from entry and clear it
        let buffer = self.imp().entry.buffer();
        let content = buffer.text().to_string();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");

        // Add new task to model
        let task = TaskObject::new(false, content);
        self.tasks().append(&task);
    }

    fn setup_actions(&self) {
        // Create action from key "filter" and add to action group "win"
        let action_filter = self.settings().create_action("filter");
        self.add_action(&action_filter);
    }

    fn remove_done_tasks(&self) {
        let tasks = self.tasks();
        let mut position = 0;
        while let Some(item) = tasks.item(position) {
            // Get `TaskObject` from `glib::Object`
            let task_object = item
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            if task_object.is_completed() {
                tasks.remove(position);
            } else {
                position += 1;
            }
        }
    }
}
