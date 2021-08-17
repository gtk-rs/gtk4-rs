use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{CompositeTemplate, Entry, ListView};
use once_cell::sync::OnceCell;

// ANCHOR: struct_and_subclass
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub list_view: TemplateChild<ListView>,
    pub model: OnceCell<gio::ListStore>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TodoWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
// ANCHOR_END: struct_and_subclass

// ANCHOR: constructed
// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        self.parent_constructed(obj);

        // Setup
        obj.setup_model();
        obj.setup_callbacks();
        obj.setup_factory();
    }
}
// ANCHOR_END: constructed

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application
impl ApplicationWindowImpl for Window {}
