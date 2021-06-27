mod imp;

use crate::todo_object::TodoObject;
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, ListView};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Window {
    pub fn new(app: &Application, list_view: &ListView, model: gio::ListStore) -> Self {
        let window: Self = Object::new(&[]).expect("Failed to create Window");

        let imp = imp::Window::from_instance(&window);

        // Set objects
        window.set_application(Some(app));
        imp.scrolled_window.set_child(Some(list_view));
        imp.model.set(model).expect("Could not set model.");

        window.connect_signals(app);

        window
    }

    fn connect_signals(&self, app: &Application) {
        let imp = imp::Window::from_instance(&self);
        let model = imp.model.get().expect("The model has to be set.");
        imp.entry
            .connect_activate(clone!(@weak model => move |entry| {
                let buffer = entry.buffer();
                let content = buffer.text();
                let todo_object = TodoObject::new(content, false);
                model
                    .append(&todo_object);
                buffer.set_text("");
            }));

        let filter_action = imp.settings.create_action("filter");
        self.add_action(&filter_action);
        app.set_accels_for_action("win.filter::All", &["<primary>a"]);
        app.set_accels_for_action("win.filter::Open", &["<primary>o"]);
        app.set_accels_for_action("win.filter::Done", &["<primary>d"]);

        // Initial filtering
        self.set_filter();

        // Filter whenever the settings key changes
        imp.settings.connect_changed(
            None,
            clone!(@weak self as self_ => move |_, key| {
                if key == "filter" {
                    self_.set_filter();
                }
            }),
        );
    }

    fn set_filter(&self) {
        let imp = imp::Window::from_instance(&self);
        let model = imp.model.get().expect("The model has to be set.");

        // Get value from settings
        let value: String = imp.settings.get("filter");

        // Set filter model accordingly
        let filter_model = match value.as_str() {
            "All" => todo!(),
            "Open" => todo!(),
            "Done" => todo!(),
            _ => unimplemented!(),
        };
    }
}
