mod imp;

use std::fs::File;

use crate::collection_object::CollectionData;
use crate::task_object::TaskObject;
use crate::utils::data_path;
use crate::APP_ID;
use adw::prelude::*;
use adw::{ActionRow, NavigationDirection};
use gio::Settings;
use glib::{clone, Object};
use gtk::glib::BindingFlags;
use gtk::subclass::prelude::*;
use gtk::Label;
use gtk::ListBoxRow;
use gtk::{gio, glib, DialogFlags, Entry, ResponseType};
use gtk::{Align, Dialog};
use gtk::{CheckButton, CustomFilter, FilterListModel, NoSelection};

use crate::collection_object::CollectionObject;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create `Window`.")
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("Could not set `Settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp().settings.get().expect("Could not get settings.")
    }

    fn current_tasks(&self) -> gio::ListStore {
        self.imp()
            .current_tasks
            .borrow()
            .clone()
            .expect("Could not get current tasks.")
    }

    fn collections(&self) -> gio::ListStore {
        self.imp()
            .collections
            .get()
            .expect("Could not get collection.")
            .clone()
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

    fn setup_collections(&self) {
        let collections = gio::ListStore::new(CollectionObject::static_type());
        self.imp()
            .collections
            .set(collections.clone())
            .expect("Could not set collections");

        self.imp()
            .collections_list
            .bind_model(Some(&collections), clone!(@weak self as window => @default-panic, move |obj| {
                let collection_object = obj.downcast_ref().expect("The object is not of type `CollectionObject`.");
                let row = window.create_collection_row(collection_object);
                row.upcast()
            }))
    }

    fn restore_data(&self) {
        if let Ok(file) = File::open(data_path()) {
            // Deserialize data from file to vector
            let backup_data: Vec<CollectionData> = serde_json::from_reader(file)
                .expect("Could not get backup data from json file.");

            // Convert `Vec<CollectionData>` to `Vec<CollectionObject>`
            let collection_objects: Vec<CollectionObject> = backup_data
                .into_iter()
                .map(CollectionObject::from_collection_data)
                .collect();

            // Insert restored objects into model
            self.collections().extend_from_slice(&collection_objects);
        }
    }

    fn create_collection_row(
        &self,
        collection_object: &CollectionObject,
    ) -> ListBoxRow {
        let label = Label::builder()
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .xalign(0.0)
            .build();

        collection_object
            .bind_property("title", &label, "label")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();

        ListBoxRow::builder().child(&label).build()
    }

    fn set_current_tasks(&self, tasks: gio::ListStore) {
        // Wrap model with filter and selection and pass it to the list view
        let filter_model = FilterListModel::new(Some(&tasks), self.filter().as_ref());
        let selection_model = NoSelection::new(Some(&filter_model));
        self.imp().tasks_list.bind_model(
            Some(&selection_model),
            clone!(@weak self as window => @default-panic, move |obj| {
                let task_object = obj.downcast_ref().expect("The object is not of type `TaskObject`.");
                let row = window.create_task_row(task_object);
                row.upcast()
            }),
        );

        // Filter model whenever the value of the key "filter" changes
        self.settings().connect_changed(
            Some("filter"),
            clone!(@weak self as window, @weak filter_model => move |_, _| {
                filter_model.set_filter(window.filter().as_ref());
            }),
        );

        // If present, disconnect old `tasks_changed` handler
        if let Some(handler_id) = self.imp().tasks_changed_handler_id.take() {
            self.current_tasks().disconnect(handler_id);
        }

        // When the items change, assure that `tasks_list` is only visible if the number of tasks is greater than 0
        tasks.connect_items_changed(
            clone!(@weak self as window => move |tasks, _, _, _| {
                window.imp().tasks_list.set_visible(tasks.n_items() > 0);
            }),
        );

        // Set current tasks
        self.imp().current_tasks.replace(Some(tasks));
    }

    fn create_task_row(&self, task_object: &TaskObject) -> ActionRow {
        let check_button = CheckButton::builder()
            .valign(Align::Center)
            .can_focus(false)
            .build();
        let row = ActionRow::builder()
            .activatable_widget(&check_button)
            .build();
        row.add_prefix(&check_button);
        task_object
            .bind_property("completed", &check_button, "active")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        task_object
            .bind_property("content", &row, "title")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        row
    }

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

        // Setup callback change of the collections
        self.collections().connect_items_changed(
            clone!(@weak self as window => move |collections, _, _, _| {
                if collections.n_items() > 0 {
                    window.imp().empty_stack.set_visible_child_name("main");
                }
                else
                {
                    window.imp().empty_stack.set_visible_child_name("empty");
                }
            }),
        );
    }

    fn new_task(&self) {
        // Get content from entry and clear it
        let buffer = self.imp().entry.buffer();
        let content = buffer.text();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");

        // Add new task to model
        let task = TaskObject::new(false, content);
        self.current_tasks().append(&task);
    }

    fn setup_actions(&self) {
        // Create action from key "filter" and add to action group "win"
        let action_filter = self.settings().create_action("filter");
        self.add_action(&action_filter);

        // Create action to remove done tasks and add to action group "win"
        let action_remove_done_tasks =
            gio::SimpleAction::new("remove-done-tasks", None);
        action_remove_done_tasks.connect_activate(
            clone!(@weak self as window => move |_, _| {
                let current_tasks = window.current_tasks();
                let mut position = 0;
                while let Some(item) = current_tasks.item(position) {
                    // Get `TaskObject` from `glib::Object`
                    let task_object = item
                        .downcast_ref::<TaskObject>()
                        .expect("The object needs to be of type `TaskObject`.");

                    if task_object.is_completed() {
                        current_tasks.remove(position);
                    } else {
                        position += 1;
                    }
                }
            }),
        );
        self.add_action(&action_remove_done_tasks);

        // Create action to create new todo list and add to action group "win"
        let action_new_list = gio::SimpleAction::new("new-collection", None);
        action_new_list.connect_activate(clone!(@weak self as window => move |_, _| {
            window.new_collection();
        }));
        self.add_action(&action_new_list);
    }

    fn new_collection(&self) {
        let dialog = Dialog::with_buttons(
            Some("New Collection"),
            Some(self),
            DialogFlags::MODAL
                | DialogFlags::DESTROY_WITH_PARENT
                | DialogFlags::USE_HEADER_BAR,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Create", ResponseType::Accept),
            ],
        );

        dialog.set_default_response(ResponseType::Accept);

        let content_area = dialog.content_area();
        let entry = Entry::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .placeholder_text("Name")
            .activates_default(true)
            .build();

        entry.connect_changed(clone!(@weak dialog => move |entry| {
            let text = entry.text();
            let button = dialog.widget_for_response(ResponseType::Accept).unwrap();
            let empty = text.is_empty();

            button.set_sensitive(!empty);

            if empty {
                entry.add_css_class("error");
            }
            else {
                entry.remove_css_class("error");
            }
        }));
        entry.remove_css_class("error");

        content_area.append(&entry);

        dialog.connect_response(
            clone!(@weak self as window, @weak entry => move |dialog, response| {
                dialog.destroy();

                if response != ResponseType::Accept {
                    return;
                }
                let title = entry.text().into();
                let tasks = gio::ListStore::new(TaskObject::static_type());
                let collection_object = CollectionObject::new(title, tasks);
                window.collections().append(&collection_object);
                window.set_current_tasks(collection_object.tasks());
                window.imp().leaflet.navigate(NavigationDirection::Forward);
            }),
        );
        dialog.present();
    }
}
