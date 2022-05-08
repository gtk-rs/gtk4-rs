use gio::Settings;
use glib::signal::Inhibit;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{CompositeTemplate, Entry, ListView, MenuButton};
use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::fs::File;

use crate::task_object::TaskObject;
use crate::utils::data_path;

// ANCHOR: struct_default
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk-rs/Todo2/window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub menu_button: TemplateChild<MenuButton>,
    #[template_child]
    pub tasks_list: TemplateChild<ListView>,
    pub current_tasks: RefCell<Option<gio::ListStore>>,
    pub settings: OnceCell<Settings>,
}
// ANCHOR_END: struct_default

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "TodoWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        self.parent_constructed(obj);

        // Setup
        obj.setup_settings();
        obj.setup_tasks();
        obj.restore_data();
        obj.setup_callbacks();
        obj.setup_factory();
        obj.setup_actions();
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for Window {}

// ANCHOR: window_impl
// Trait shared by all windows
impl WindowImpl for Window {
    fn close_request(&self, window: &Self::Type) -> Inhibit {
        // Store todo data in vector
        let mut backup_data = Vec::new();
        let mut position = 0;
        while let Some(item) = window.current_tasks().item(position) {
            // Get `TodoObject` from `glib::Object`
            let todo_data = item
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TodoObject`.")
                .task_data();
            // Add todo data to vector and increase position
            backup_data.push(todo_data);
            position += 1;
        }

        // Save state in file
        let file = File::create(data_path()).expect("Could not create json file.");
        serde_json::to_writer(file, &backup_data)
            .expect("Could not write data to json file");

        // Pass close request on to the parent
        self.parent_close_request(window)
    }
}
// ANCHOR_END: window_impl

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
