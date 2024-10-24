mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct ExApplicationWindow(ObjectSubclass<imp::ExApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup;
}

impl ExApplicationWindow {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn init_label(&self) {
        // To access fields such as template children, you must get
        // the private struct.
        let imp = self.imp();
        imp.subtitle
            .set_text("This is an example window made using composite templates");
    }
}
