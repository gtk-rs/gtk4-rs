mod imp;

use std::fs::File;

use adw::prelude::*;
use adw::{ActionRow, NavigationDirection};
use gio::Settings;
use glib::{clone, Object};
use gtk::glib::BindingFlags;
use gtk::subclass::prelude::*;
use gtk::{
    gio, glib, pango, Align, CheckButton, CustomFilter, Dialog, DialogFlags, Entry,
    FilterListModel, Label, ListBoxRow, NoSelection, ResponseType, SelectionMode,
};

use crate::collection_object::{CollectionData, CollectionObject};
use crate::task_object::TaskObject;
use crate::utils::data_path;
use crate::APP_ID;

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

    fn current_collection(&self) -> CollectionObject {
        self.imp()
            .current_collection
            .borrow()
            .clone()
            .expect("Could not get current collection.")
    }

    fn tasks(&self) -> gio::ListStore {
        self.current_collection().tasks()
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
            let collections: Vec<CollectionObject> = backup_data
                .into_iter()
                .map(CollectionObject::from_collection_data)
                .collect();

            // Insert restored objects into model
            self.collections().extend_from_slice(&collections);

            // Set first collection as current
            if let Some(first_collection) = collections.first() {
                self.set_current_collection(first_collection.clone());
            }
        }
    }

    fn create_collection_row(
        &self,
        collection_object: &CollectionObject,
    ) -> ListBoxRow {
        let label = Label::builder()
            .ellipsize(pango::EllipsizeMode::End)
            .xalign(0.0)
            .build();

        collection_object
            .bind_property("title", &label, "label")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();

        ListBoxRow::builder().child(&label).build()
    }

    fn set_current_collection(&self, collection: CollectionObject) {
        // Wrap model with filter and selection and pass it to the list view
        let tasks = collection.tasks();
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
            self.tasks().disconnect(handler_id);
        }

        // Assure that the task list is only visible when it is supposed to
        self.set_task_list_visible(&tasks);
        let tasks_changed_handler_id = tasks.connect_items_changed(
            clone!(@weak self as window => move |tasks, _, _, _| {
                window.set_task_list_visible(tasks);
            }),
        );
        self.imp()
            .tasks_changed_handler_id
            .replace(Some(tasks_changed_handler_id));

        // Set current tasks
        self.imp().current_collection.replace(Some(collection));

        self.select_current_row();
    }

    /// Assure that `tasks_list` is only visible if the number of tasks is greater than 0
    fn set_task_list_visible(&self, tasks: &gio::ListStore) {
        self.imp().tasks_list.set_visible(tasks.n_items() > 0);
    }

    fn select_current_row(&self) {
        if let Some(index) = self.collections().find(&self.current_collection()) {
            let row = self.imp().collections_list.row_at_index(index as i32);
            self.imp().collections_list.select_row(row.as_ref());
        }
    }

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
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        task_object
            .bind_property("content", &row, "title")
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        // Return row
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
        self.set_stack();
        self.collections().connect_items_changed(
            clone!(@weak self as window => move |_, _, _, _| {
                window.set_stack();
            }),
        );

        // Setup callback for activating a row of collections list
        self.imp().collections_list.connect_row_activated(
            clone!(@weak self as window => move |_, row| {
                let index = row.index();
                let selected_collection = window.collections()
                    .item(index as u32)
                    .expect("There needs to be an object at this position.")
                    .downcast::<CollectionObject>()
                    .expect("The object needs to be a `CollectionObject`.");
                window.set_current_collection(selected_collection);
                window.imp().leaflet.navigate(NavigationDirection::Forward);
            }),
        );

        // Setup callback for folding the leaflet
        self.imp().leaflet.connect_folded_notify(clone!(@weak self as window => move |leaflet| {
            if leaflet.is_folded() {
                window.imp().collections_list.set_selection_mode(SelectionMode::None)
            }
            else {
                window.imp().collections_list.set_selection_mode(SelectionMode::Single);
                window.select_current_row();
            }
        }));

        self.imp().back_button.connect_clicked(
            clone!(@weak self as window => move |_| {
                window.imp().leaflet.navigate(NavigationDirection::Back);
            }),
        );
    }

    fn set_stack(&self) {
        if self.collections().n_items() > 0 {
            self.imp().stack.set_visible_child_name("main");
        } else {
            self.imp().stack.set_visible_child_name("empty");
        }
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
        self.tasks().append(&task);
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
                let tasks = window.tasks();
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
            }),
        );
        self.add_action(&action_remove_done_tasks);

        // Create action to remove current collection and add to action group "win"
        let action_remove_current_collection =
            gio::SimpleAction::new("remove-current-collection", None);
        action_remove_current_collection.connect_activate(
            clone!(@weak self as window => move |_, _| {
                if let Some(index) = window.collections().find(&window.current_collection()) {
                    window.collections().remove(index);
                }
            }),
        );
        self.add_action(&action_remove_current_collection);

        // Create action to create new todo list and add to action group "win"
        let action_new_list = gio::SimpleAction::new("new-collection", None);
        action_new_list.connect_activate(clone!(@weak self as window => move |_, _| {
            window.new_collection();
        }));
        self.add_action(&action_new_list);
    }

    fn new_collection(&self) {
        // Create new Dialog
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

        // Set dialog button initially to false
        let dialog_button = dialog
            .widget_for_response(ResponseType::Accept)
            .expect("The dialog needs to have a widget for response type `Accept`.");
        dialog_button.set_sensitive(false);

        // Create entry and add it to the dialog
        let entry = Entry::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .placeholder_text("Name")
            .activates_default(true)
            .build();
        dialog.content_area().append(&entry);

        // Set entry's css class to "error", when there is not text in it
        entry.connect_changed(clone!(@weak dialog => move |entry| {
            let text = entry.text();
            let dialog_button = dialog.
                widget_for_response(ResponseType::Accept).
                expect("The dialog needs to have a widget for response type `Accept`.");
            let empty = text.is_empty();

            dialog_button.set_sensitive(!empty);

            if empty {
                entry.add_css_class("error");
            }
            else {
                entry.remove_css_class("error");
            }
        }));

        // Connect response to dialog
        dialog.connect_response(
            clone!(@weak self as window, @weak entry => move |dialog, response| {
                // Destroy dialog
                dialog.destroy();

                // Return if the user chose a response different than `Accept`
                if response != ResponseType::Accept {
                    return;
                }

                // Create a new list store
                let tasks = gio::ListStore::new(TaskObject::static_type());

                // Create a new collection object from the title the user provided
                let title = entry.text().into();
                let collection = CollectionObject::new(title, tasks);

                // Add new collection object and set current tasks
                window.collections().append(&collection);
                window.set_current_collection(collection);

                // Let the leaflet navigate to the next child
                window.imp().leaflet.navigate(NavigationDirection::Forward);
            }),
        );
        dialog.present();
    }
}
