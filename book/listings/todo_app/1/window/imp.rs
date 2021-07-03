use std::fs::File;

use gio::Settings;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{Button, CompositeTemplate, Entry, ListView, MenuButton};
use once_cell::sync::OnceCell;

use super::data_path;
use crate::todo_object::TodoObject;

// Object holding the state
#[derive(CompositeTemplate)]
#[template(file = "window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub menu_button: TemplateChild<MenuButton>,
    #[template_child]
    pub list_view: TemplateChild<ListView>,
    #[template_child]
    pub clear_button: TemplateChild<Button>,
    pub settings: Settings,
    pub model: OnceCell<gio::ListStore>,
}

impl Default for Window {
    fn default() -> Self {
        Window {
            entry: TemplateChild::default(),
            menu_button: TemplateChild::default(),
            list_view: TemplateChild::default(),
            clear_button: TemplateChild::default(),
            settings: Settings::new("org.gtk.example"),
            model: OnceCell::new(),
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
impl WindowImpl for Window {
    fn close_request(&self, window: &Self::Type) -> glib::signal::Inhibit {
        // Get model
        let model = self.model.get().expect("Could not get model");

        // Store todo data in vector
        let mut backup_data = Vec::new();
        let mut position = 0;
        while let Some(item) = model.item(position) {
            // Get `TodoObject` from `glib::Object`
            let todo_data = item
                .downcast_ref::<TodoObject>()
                .expect("The object needs to be of type `TodoObject`.")
                .todo_data();

            backup_data.push(todo_data);
            position += 1;
        }

        // Save state in file
        let file = File::create(data_path()).expect("Could not create json file.");
        serde_json::to_writer_pretty(file, &backup_data)
            .expect("Could not write data to json file");

        // Pass close request on to the parent
        self.parent_close_request(window)
    }
}

// Trait shared by all application
impl ApplicationWindowImpl for Window {}
