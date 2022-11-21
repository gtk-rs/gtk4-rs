mod imp;

use gio::SimpleAction;
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

// ANCHOR: impl_window
impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn setup_actions(&self) {
        let label = self.imp().label.get();

        // Add stateful action "count" to `window` taking an integer as parameter
        let original_state = 0;
        let action_count = SimpleAction::new_stateful(
            "count",
            Some(&i32::static_variant_type()),
            &original_state.to_variant(),
        );

        action_count.connect_activate(clone!(@weak label => move |action, parameter| {
            // Get state
            let mut state = action
                .state()
                .expect("Could not get state.")
                .get::<i32>()
                .expect("The value needs to be of type `i32`.");

            // Get parameter
            let parameter = parameter
                .expect("Could not get parameter.")
                .get::<i32>()
                .expect("The value needs to be of type `i32`.");

            // Increase state by parameter and save state
            state += parameter;
            action.set_state(&state.to_variant());

            // Update label with new state
            label.set_label(&format!("Counter: {state}"));
        }));
        self.add_action(&action_count);
    }
}
// ANCHOR_END: impl_window
