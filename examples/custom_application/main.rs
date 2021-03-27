use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;
    use gtk::subclass::prelude::*;

    #[derive(Debug, Default)]
    // By implementing Default we don't have to provide a `new` fn in our ObjectSubclass impl.
    pub struct ExampleApp {}

    #[glib::object_subclass]
    impl ObjectSubclass for ExampleApp {
        const NAME: &'static str = "ExampleApp";
        type Type = super::ExampleApp;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for ExampleApp {}
    impl ApplicationImpl for ExampleApp {
        fn activate(&self, application: &Self::Type) {
            // We create our window at activation stage
            let window = gtk::ApplicationWindow::new(application);
            window.set_default_size(600, 350);
            window.set_title(Some("Application Subclass"));

            let label = gtk::Label::new(Some("Hello"));
            label.add_css_class("title-2");
            window.set_child(Some(&label));
            window.show();
        }
    }
    impl GtkApplicationImpl for ExampleApp {}
}

glib::wrapper! {
    pub struct ExampleApp(ObjectSubclass<imp::ExampleApp>) @extends gio::Application, gtk::Application, @implements gio::ActionGroup, gio::ActionMap;
}

impl ExampleApp {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &"org.gtk_rs.application-subclass"),
            ("flags", &gio::ApplicationFlags::empty()),
        ])
        .expect("Failed to create ExampleApp")
    }
}

fn main() {
    let app = ExampleApp::new();
    let argv = std::env::args().collect::<Vec<_>>();
    std::process::exit(app.run(&argv));
}
