mod imp;

use std::fs::File;

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{ActionRow, NavigationDirection};
use gio::Settings;
use glib::{clone, Object};
use gtk::glib::BindingFlags;
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

    // ANCHOR: helper
    fn tasks(&self) -> gio::ListStore {
        self.current_collection().tasks()
    }

    fn current_collection(&self) -> CollectionObject {
        self.imp()
            .current_collection
            .borrow()
            .clone()
            .expect("`current_collection` should be set in `set_current_collections`.")
    }

    fn collections(&self) -> gio::ListStore {
        self.imp()
            .collections
            .get()
            .expect("`collections` should be set in `setup_collections`.")
            .clone()
    }

    fn set_filter(&self) {
        self.imp()
            .current_filter_model
            .borrow()
            .clone()
            .expect("`current_filter_model` should be set in `set_current_collection`.")
            .set_filter(self.filter().as_ref());
    }
    // ANCHOR_END: helper

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

    // ANCHOR: setup_collections
    fn setup_collections(&self) {
        let collections = gio::ListStore::new(CollectionObject::static_type());
        self.imp()
            .collections
            .set(collections.clone())
            .expect("Could not set collections");

        self.imp().collections_list.bind_model(
            Some(&collections),
            clone!(@weak self as window => @default-panic, move |obj| {
                let collection_object = obj
                    .downcast_ref()
                    .expect("The object should be of type `CollectionObject`.");
                let row = window.create_collection_row(collection_object);
                row.upcast()
            }),
        )
    }
    // ANCHOR_END: setup_collections

    // ANCHOR: restore_data
    fn restore_data(&self) {
        if let Ok(file) = File::open(data_path()) {
            // Deserialize data from file to vector
            let backup_data: Vec<CollectionData> = serde_json::from_reader(file)
                .expect(
                    "It should be possible to read `backup_data` from the json file.",
                );

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
    // ANCHOR_END: restore_data

    // ANCHOR: create_collection_row
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
    // ANCHOR_END: create_collection_row

    // ANCHOR: set_current_collection
    fn set_current_collection(&self, collection: CollectionObject) {
        // Wrap model with filter and selection and pass it to the list box
        let tasks = collection.tasks();
        let filter_model = FilterListModel::new(Some(&tasks), self.filter().as_ref());
        let selection_model = NoSelection::new(Some(&filter_model));
        self.imp().tasks_list.bind_model(
            Some(&selection_model),
            clone!(@weak self as window => @default-panic, move |obj| {
                let task_object = obj
                    .downcast_ref()
                    .expect("The object should be of type `TaskObject`.");
                let row = window.create_task_row(task_object);
                row.upcast()
            }),
        );

        // Store filter model
        self.imp().current_filter_model.replace(Some(filter_model));

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

        self.select_collection_row();
    }
    // ANCHOR_END: set_current_collection

    // ANCHOR: set_task_list_visible
    fn set_task_list_visible(&self, tasks: &gio::ListStore) {
        self.imp().tasks_list.set_visible(tasks.n_items() > 0);
    }
    // ANCHOR_END: set_task_list_visible

    // ANCHOR: select_collection_row
    fn select_collection_row(&self) {
        if let Some(index) = self.collections().find(&self.current_collection()) {
            let row = self.imp().collections_list.row_at_index(index as i32);
            self.imp().collections_list.select_row(row.as_ref());
        }
    }
    // ANCHOR_END: select_collection_row

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

        // ANCHOR: setup_callbacks
        // Filter model whenever the value of the key "filter" changes
        self.settings().connect_changed(
            Some("filter"),
            clone!(@weak self as window => move |_, _| {
                window.set_filter();
            }),
        );

        // Setup callback when items of collections change
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
        self.imp().leaflet.connect_folded_notify(
            clone!(@weak self as window => move |leaflet| {
                if leaflet.is_folded() {
                    window
                        .imp()
                        .collections_list
                        .set_selection_mode(SelectionMode::None)
                } else {
                    window
                        .imp()
                        .collections_list
                        .set_selection_mode(SelectionMode::Single);
                    window.select_collection_row();
                }
            }),
        );

        self.imp().back_button.connect_clicked(
            clone!(@weak self as window => move |_| {
                window.imp().leaflet.navigate(NavigationDirection::Back);
            }),
        );
        // ANCHOR_END: setup_callbacks
    }

    // ANCHOR: set_stack
    fn set_stack(&self) {
        if self.collections().n_items() > 0 {
            self.imp().stack.set_visible_child_name("main");
        } else {
            self.imp().stack.set_visible_child_name("placeholder");
        }
    }
    // ANCHOR_END: set_stack

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

        // ANCHOR: setup_actions
        // Create action to create new collection and add to action group "win"
        let action_new_list = gio::SimpleAction::new("new-collection", None);
        action_new_list.connect_activate(clone!(@weak self as window => move |_, _| {
            window.new_collection();
        }));
        self.add_action(&action_new_list);
        // ANCHOR_END: setup_actions
    }

    // ANCHOR: new_collection
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

        // Make the dialog button insensitive initially
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
            } else {
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
                let title = entry.text().to_string();
                let collection = CollectionObject::new(&title, tasks);

                // Add new collection object and set current tasks
                window.collections().append(&collection);
                window.set_current_collection(collection);

                // Let the leaflet navigate to the next child
                window.imp().leaflet.navigate(NavigationDirection::Forward);
            }),
        );
        dialog.present();
    }
    // ANCHOR_END: new_collection
}
