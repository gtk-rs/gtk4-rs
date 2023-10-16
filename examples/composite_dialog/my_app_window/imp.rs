use std::cell::Cell;

use gtk::{glib, prelude::*, subclass::prelude::*};

// Object holding the state
#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(file = "window.ui")]
#[properties(wrapper_type = super::MyAppWindow)]
pub struct MyAppWindow {
    #[property(get, set)]
    counter: Cell<i32>,
    #[template_child]
    pub count_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub plus: TemplateChild<gtk::Button>,
    #[template_child]
    pub minus: TemplateChild<gtk::Button>,
    #[template_child]
    pub popup: TemplateChild<gtk::Dialog>,
}

#[glib::object_subclass]
impl ObjectSubclass for MyAppWindow {
    const NAME: &'static str = "MyAppWindow";
    type Type = super::MyAppWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for MyAppWindow {}

impl WidgetImpl for MyAppWindow {}
impl WindowImpl for MyAppWindow {}
impl ApplicationWindowImpl for MyAppWindow {}
