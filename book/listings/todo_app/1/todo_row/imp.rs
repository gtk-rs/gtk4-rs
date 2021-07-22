use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, Label};
use std::cell::RefCell;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(file = "todo_row.ui")]
pub struct TodoRow {
    #[template_child]
    pub completed_button: TemplateChild<CheckButton>,
    #[template_child]
    pub content_label: TemplateChild<Label>,
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TodoRow {
    const NAME: &'static str = "MyGtkAppTodoRow";
    type Type = super::TodoRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for TodoRow {}

// Trait shared by all widgets
impl WidgetImpl for TodoRow {}

// Trait shared by all boxes
impl BoxImpl for TodoRow {}
