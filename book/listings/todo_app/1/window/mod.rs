mod imp;

use crate::todo_object::TodoObject;
use glib::Object;
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

        // Set application and list view
        window.set_application(Some(app));
        imp.scrolled_window.set_child(Some(list_view));

        imp.entry.connect_activate(move |entry| {
            let buffer = entry.buffer();
            let content = buffer.text();
            let todo_object = TodoObject::new(content, false);
            model.append(&todo_object);
            buffer.set_text("");
        });

        window
    }
}
