use gio::Settings;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};

// Object holding the state
#[derive(CompositeTemplate)]
#[template(file = "window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<gtk::Entry>,
    #[template_child]
    pub menu_button: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub list_view: TemplateChild<gtk::ListView>,
    #[template_child]
    pub clear_button: TemplateChild<gtk::Button>,
    pub settings: Settings,
}

impl Default for Window {
    fn default() -> Self {
        Window {
            entry: TemplateChild::default(),
            menu_button: TemplateChild::default(),
            list_view: TemplateChild::default(),
            clear_button: TemplateChild::default(),
            settings: Settings::new("org.gtk.example"),
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
