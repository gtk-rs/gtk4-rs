use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::ex_menu_button::ExMenuButton;

/// The private struct, which can hold widgets and other data.
#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "ex_application_window.ui")]
pub struct ExApplicationWindow {
    // The #[template_child] attribute tells the CompositeTemplate macro
    // that a field is meant to be a child within the template.
    #[template_child]
    pub headerbar: TemplateChild<gtk::HeaderBar>,
    #[template_child]
    pub label: TemplateChild<gtk::Label>,
    // You can specify the optional `id` argument if the id is not the same
    // as the identifier
    #[template_child(id = "subtitle_label")]
    pub subtitle: TemplateChild<gtk::Label>,
    #[template_child]
    pub menubutton: TemplateChild<ExMenuButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for ExApplicationWindow {
    const NAME: &'static str = "ExApplicationWindow";
    type Type = super::ExApplicationWindow;
    type ParentType = gtk::ApplicationWindow;

    // Within class_init() you must set the template.
    // The CompositeTemplate derive macro provides a convenience function
    // bind_template() to set the template and bind all children at once.
    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        UtilityCallbacks::bind_template_callbacks(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

struct UtilityCallbacks {}

#[gtk::template_callbacks(functions)]
impl UtilityCallbacks {
    #[template_callback]
    fn to_string(value: &glib::Value) -> String {
        if let Ok(value) = value.get::<u64>() {
            value.to_string()
        } else if let Ok(value) = value.get::<&str>() {
            value.to_owned()
        } else {
            "".into()
        }
    }
    #[template_callback]
    fn strlen(s: &str) -> u64 {
        s.len() as u64
    }
    #[template_callback(name = "concat_strs")]
    fn concat(#[rest] values: &[glib::Value]) -> String {
        let mut res = String::new();
        for (index, value) in values.iter().enumerate() {
            res.push_str(value.get::<&str>().unwrap_or_else(|e| {
                panic!("Expected string value for argument {}: {}", index, e);
            }));
        }
        res
    }
    #[template_callback(function = false, name = "reset_entry")]
    fn reset(entry: &gtk::Entry) {
        entry.set_text("Nothing");
    }
}

impl ObjectImpl for ExApplicationWindow {
    fn constructed(&self, obj: &Self::Type) {
        obj.init_label();
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for ExApplicationWindow {}
impl WindowImpl for ExApplicationWindow {}
impl ApplicationWindowImpl for ExApplicationWindow {}
