use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::CompositeTemplate;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "example_app_row.ui")]
pub struct ExampleAppRow {
    #[template_child]
    pub name: TemplateChild<gtk::Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for ExampleAppRow {
    const NAME: &'static str = "ExampleAppRow";
    type Type = super::ExampleAppRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ExampleAppRow {}
impl WidgetImpl for ExampleAppRow {}
impl BoxImpl for ExampleAppRow {}
