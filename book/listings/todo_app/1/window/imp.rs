use gio::{ListStore, Settings};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};
use once_cell::sync::OnceCell;

// Object holding the state
#[derive(CompositeTemplate)]
#[template(file = "window.ui")]
pub struct Window {
    // Template childs
    #[template_child]
    pub entry: TemplateChild<gtk::Entry>,
    #[template_child]
    pub menu_button: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub scrolled_window: TemplateChild<gtk::ScrolledWindow>,
    // Other state
    pub model: OnceCell<ListStore>,
    pub settings: Settings,
}

impl Default for Window {
    fn default() -> Self {
        Window {
            model: OnceCell::new(),
            settings: Settings::new("org.gtk.TodoApp"),
            ..Default::default()
        }
    }
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application
impl ApplicationWindowImpl for Window {}
