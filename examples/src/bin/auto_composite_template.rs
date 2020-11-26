//! # Composite Template Example
//!
//! This sample demonstrates how to create a widget using GTK's composite templates.

use gio::prelude::*;
use glib::subclass::prelude::*;
use glib::{glib_object_subclass, glib_wrapper};
use gtk::gtk4_macros::*;
use gtk::prelude::*;

use gtk::subclass::prelude::*;
use gtk::subclass::widget::*;

mod imp {
    use super::*;

    /// The private struct, which will hold the automatically generated widgets
    #[template(file = "examples/src/bin/composite_template.ui")]
    #[derive(Debug)]
    pub struct ExApplicationWindow {}
}

glib_wrapper! {
    pub struct ExApplicationWindow(ObjectSubclass<imp::ExApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl WidgetImpl for imp::ExApplicationWindow {}
impl WindowImpl for imp::ExApplicationWindow {}
impl ApplicationWindowImpl for imp::ExApplicationWindow {}

impl ExApplicationWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(Self::static_type(), &[("application", app)])
            .expect("Failed to create ExApplicationWindow")
            .downcast::<ExApplicationWindow>()
            .expect("Created object is of wrong type")
    }

    pub fn init_label(&self) {
        // To access fields such as template children, you must get
        // the private struct.
        let self_ = imp::ExApplicationWindow::from_instance(self);
        self_
            .subtitle_label
            .get()
            .set_text("This is an example window made using composite templates");
    }
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.composite_template"),
        Default::default(),
    )
    .expect("Failed to initialize application");

    application.connect_activate(|app| {
        let win = ExApplicationWindow::new(app);
        win.show();
    });

    application.run(&std::env::args().collect::<Vec<_>>());
}
