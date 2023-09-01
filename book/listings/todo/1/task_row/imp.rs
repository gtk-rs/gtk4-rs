use std::cell::RefCell;

use glib::Binding;
use gtk::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, Label};

// ANCHOR: struct_and_subclass
// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/Todo1/task_row.ui")]
pub(crate) struct TaskRow {
    #[template_child]
    pub(crate) completed_button: TemplateChild<CheckButton>,
    #[template_child]
    pub(crate) content_label: TemplateChild<Label>,
    // Vector holding the bindings to properties of `TaskObject`
    pub(crate) bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TaskRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "TodoTaskRow";
    type Type = super::TaskRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
// ANCHOR_END: struct_and_subclass

// Trait shared by all GObjects
impl ObjectImpl for TaskRow {}

// Trait shared by all widgets
impl WidgetImpl for TaskRow {}

// Trait shared by all boxes
impl BoxImpl for TaskRow {}
