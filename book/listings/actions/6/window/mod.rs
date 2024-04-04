mod imp;

use gio::{ActionEntry, PropertyAction};
use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, Orientation};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    // ANCHOR: setup_actions
    fn setup_actions(&self) {
        // Add stateful action "count" to `window` taking an integer as parameter
        let original_state = 0;
        let action_count = ActionEntry::builder("count")
            .parameter_type(Some(&i32::static_variant_type()))
            .state(original_state.to_variant())
            .activate(move |window: &Self, action, parameter| {
                // Get state
                let mut state = action
                    .state()
                    .expect("Could not get state.")
                    .get::<i32>()
                    .expect("The variant needs to be of type `i32`.");

                // Get parameter
                let parameter = parameter
                    .expect("Could not get parameter.")
                    .get::<i32>()
                    .expect("The variant needs to be of type `i32`.");

                // Increase state by parameter and store state
                state += parameter;
                action.set_state(&state.to_variant());

                // Update label with new state
                window.imp().label.set_label(&format!("Counter: {state}"));
            })
            .build();
        // ANCHOR: action_button_frame
        // Add property action "button-frame" to `window`
        let button = self.imp().button.get();
        let action_button_frame =
            PropertyAction::new("button-frame", &button, "has-frame");
        self.add_action(&action_button_frame);
        // ANCHOR_END: action_button_frame

        // ANCHOR: action_orientation
        // Add stateful action "orientation" to `window` taking a string as parameter
        let action_orientation = ActionEntry::builder("orientation")
            .parameter_type(Some(&String::static_variant_type()))
            .state("Vertical".to_variant())
            .activate(move |window: &Self, action, parameter| {
                // Get parameter
                let parameter = parameter
                    .expect("Could not get parameter.")
                    .get::<String>()
                    .expect("The value needs to be of type `String`.");

                let orientation = match parameter.as_str() {
                    "Horizontal" => Orientation::Horizontal,
                    "Vertical" => Orientation::Vertical,
                    _ => unreachable!(),
                };

                // Set orientation and save state
                window.imp().gtk_box.set_orientation(orientation);
                action.set_state(&parameter.to_variant());
            })
            .build();
        self.add_action_entries([action_count, action_orientation]);
        //ANCHOR_END: action_orientation
    }
    // ANCHOR_END: setup_actions
}
