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

        window.connect_signals();

        window
    }

    fn connect_signals(&self) {
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

        imp.settings.connect_changed(
            None,
            clone!(@weak self as self_ => move |_, key| {
                self_.set_filter(key);
            }),
        );
        self.set_filter("filter");
    }

    fn set_filter(&self, key: &str) {
        // 1. Gets value from settings
        // 2. Sets model accordingly
    }
}
