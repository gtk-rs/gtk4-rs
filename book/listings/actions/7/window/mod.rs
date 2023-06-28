mod imp;

use gio::{Settings, SimpleAction};
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, Orientation};

use crate::APP_ID;

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

    // ANCHOR: settings
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
    // ANCHOR_END: settings

    fn setup_actions(&self) {
        // Get state
        let label = self.imp().label.get();

        // Add stateful action "count" to `window` taking an integer as parameter
        let original_state = 0;
        let action_count = SimpleAction::new_stateful(
            "count",
            Some(&i32::static_variant_type()),
            original_state.to_variant(),
        );

        action_count.connect_activate(clone!(@weak label => move |action, parameter| {
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

            // Increase state by parameter and save state
            state += parameter;
            action.set_state(state.to_variant());

            // Update label with new state
            label.set_label(&format!("Counter: {state}"));
        }));
        self.add_action(&action_count);

        // Add action "close" to `window` taking no parameter
        let action_close = SimpleAction::new("close", None);

        action_close.connect_activate(clone!(@weak self as window => move |_, _| {
            window.close();
        }));
        self.add_action(&action_close);

        // ANCHOR: settings_create_actions
        // Create action from key "toggle-button-frame" and add to action group "win"
        let action_toggle_button_frame =
            self.settings().create_action("toggle-button-frame");
        self.add_action(&action_toggle_button_frame);

        // Create action from key "orientation" and add to action group "win"
        let action_orientation = self.settings().create_action("orientation");
        self.add_action(&action_orientation);
        // ANCHOR_END: settings_create_actions
    }

    // ANCHOR: bind_settings
    fn bind_settings(&self) {
        // Get state

        // Bind setting "toggle-button-frame" to "has-frame" property of `button`
        let button = self.imp().button.get();
        self.settings()
            .bind("toggle-button-frame", &button, "has-frame")
            .build();

        // Bind setting "orientation" to "orientation" property of `button`
        let gtk_box = self.imp().gtk_box.get();
        self.settings()
            .bind("orientation", &gtk_box, "orientation")
            .mapping(|variant, _| {
                let orientation = variant
                    .get::<String>()
                    .expect("The variant needs to be of type `String`.");

                let orientation = match orientation.as_str() {
                    "Horizontal" => Orientation::Horizontal,
                    "Vertical" => Orientation::Vertical,
                    _ => unreachable!(),
                };

                Some(orientation.to_value())
            })
            .build();
    }
    // ANCHOR_END: bind_settings
}
