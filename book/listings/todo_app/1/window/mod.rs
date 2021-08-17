mod imp;

use crate::todo_object::TodoObject;
use crate::todo_row::TodoRow;
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

    // ANCHOR: model
    fn model(&self) -> &gio::ListStore {
        // Get state
        let imp = imp::Window::from_instance(self);
        imp.model.get().expect("Could not get model")
    }

    fn setup_model(&self) {
        // Create new model
        let model = gio::ListStore::new(TodoObject::static_type());

        // Get state and set model
        let imp = imp::Window::from_instance(self);
        imp.model.set(model).expect("Could not set model");

        // Wrap model with selection and pass it to the list view
        let selection_model = NoSelection::new(Some(self.model()));
        imp.list_view.set_model(Some(&selection_model));
    }
    // ANCHOR_END: model

    // ANCHOR: setup_callbacks
    fn setup_callbacks(&self) {
        // Get state
        let imp = imp::Window::from_instance(self);
        let model = self.model();

        // Setup callback so that activation of the entry
        // creates a new todo object and clears the entry
        imp.entry
            .connect_activate(clone!(@weak model => move |entry| {
                let buffer = entry.buffer();
                let content = buffer.text();
                let todo_object = TodoObject::new(content, false);
                model.append(&todo_object);
                buffer.set_text("");
            }));
    }
    // ANCHOR_END: setup_callbacks

    // ANCHOR: setup_factory
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
        let imp = imp::Window::from_instance(self);
        imp.list_view.set_factory(Some(&factory));
    }
    // ANCHOR_END: setup_factory
}
