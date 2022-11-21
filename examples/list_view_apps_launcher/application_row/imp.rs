use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::CompositeTemplate;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "application_row.ui")]
pub struct ApplicationRow {
    #[template_child]
    pub name: TemplateChild<gtk::Label>,
    #[template_child]
    pub description: TemplateChild<gtk::Label>,
    #[template_child]
    pub image: TemplateChild<gtk::Image>,
}

#[glib::object_subclass]
impl ObjectSubclass for ApplicationRow {
    const NAME: &'static str = "ApplicationRow";
    type Type = super::ApplicationRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ApplicationRow {}
impl WidgetImpl for ApplicationRow {}
impl BoxImpl for ApplicationRow {}
