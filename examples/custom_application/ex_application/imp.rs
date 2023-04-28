use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Debug, Default)]
// By implementing Default we don't have to provide a `new` fn in our ObjectSubclass impl.
pub struct ExApplication {}

#[glib::object_subclass]
impl ObjectSubclass for ExApplication {
    const NAME: &'static str = "ExApplication";
    type Type = super::ExApplication;
    type ParentType = gtk::Application;
}

impl ObjectImpl for ExApplication {}
impl ApplicationImpl for ExApplication {
    fn activate(&self) {
        self.parent_activate();
        // We create our window at activation stage
        let window = gtk::ApplicationWindow::new(&*self.obj());
        window.set_default_size(600, 350);
        window.set_title(Some("Application Subclass"));

        let label = gtk::Label::new(Some("Hello"));
        label.add_css_class("title-2");
        window.set_child(Some(&label));
        window.present();
    }
}
impl GtkApplicationImpl for ExApplication {}
