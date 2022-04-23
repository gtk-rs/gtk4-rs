mod imp;

use crate::task_object::TaskObject;
use crate::task_row::TaskRow;
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, NoSelection, SignalListItemFactory};

// ANCHOR: glib_wrapper
glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}
// ANCHOR_END: glib_wrapper

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create `Window`.")
    }

    // ANCHOR: tasks
    fn current_tasks(&self) -> &gio::ListStore {
        // Get state
        self.imp().current_tasks.get().expect("Could not get model")
    }

    fn setup_tasks(&self) {
        // Create new model
        let model = gio::ListStore::new(TaskObject::static_type());

        // Get state and set model
        self.imp()
            .current_tasks
            .set(model)
            .expect("Could not set model");

        // Wrap model with selection and pass it to the list view
        let selection_model = NoSelection::new(Some(self.current_tasks()));
        self.imp().list_view.set_model(Some(&selection_model));
    }
    // ANCHOR_END: tasks

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

    // ANCHOR: new_task
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
    // ANCHOR_END: new_task

    // ANCHOR: setup_factory
    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `TaskRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `TaskRow`
            let task_row = TaskRow::new();
            list_item.set_child(Some(&task_row));
        });

        // Tell factory how to bind `TaskRow` to a `TaskObject`
        factory.connect_bind(move |_, list_item| {
            // Get `TaskObject` from `ListItem`
            let task_object = list_item
                .item()
                .expect("The item has to exist.")
                .downcast::<TaskObject>()
                .expect("The item has to be an `TaskObject`.");

            // Get `TaskRow` from `ListItem`
            let task_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TaskRow>()
                .expect("The child has to be a `TaskRow`.");

            task_row.bind(&task_object);
        });

        // Tell factory how to unbind `TaskRow` from `TaskObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `TaskRow` from `ListItem`
            let task_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TaskRow>()
                .expect("The child has to be a `TaskRow`.");

            task_row.unbind();
        });

        // Set the factory of the list view
        self.imp().list_view.set_factory(Some(&factory));
    }
    // ANCHOR_END: setup_factory
}
