mod imp;

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
    pub fn new(app: &Application, list_view: &ListView) -> Self {
        let window: Self = Object::new(&[]).expect("Failed to create Window");
        let imp = imp::Window::from_instance(&window);

        // Set application and list view
        window.set_application(Some(app));
        imp.scrolled_window.set_child(Some(list_view));

        window
    }
}
