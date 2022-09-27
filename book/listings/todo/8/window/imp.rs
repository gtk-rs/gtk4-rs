use std::cell::RefCell;
use std::fs::File;

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::Leaflet;
use gio::Settings;
use glib::signal::Inhibit;
use glib::subclass::InitializingObject;
use gtk::glib::SignalHandlerId;
use gtk::{
    gio, glib, Button, CompositeTemplate, Entry, FilterListModel, ListBox, Stack,
};
use once_cell::sync::OnceCell;

use crate::collection_object::{CollectionData, CollectionObject};
use crate::utils::data_path;

// ANCHOR: struct
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/Todo8/window.ui")]
pub struct Window {
    pub settings: OnceCell<Settings>,
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub tasks_list: TemplateChild<ListBox>,
    // 👇 all members below are new
    #[template_child]
    pub collections_list: TemplateChild<ListBox>,
    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,
    #[template_child]
    pub stack: TemplateChild<Stack>,
    #[template_child]
    pub back_button: TemplateChild<Button>,
    pub collections: OnceCell<gio::ListStore>,
    pub current_collection: RefCell<Option<CollectionObject>>,
    pub current_filter_model: RefCell<Option<FilterListModel>>,
    pub tasks_changed_handler_id: RefCell<Option<SignalHandlerId>>,
}
// ANCHOR_END: struct

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "TodoWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

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
        obj.setup_collections();
        obj.restore_data();
        obj.setup_callbacks();
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
        // Store task data in vector
        let backup_data: Vec<CollectionData> = window
            .collections()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<CollectionObject>)
            .map(CollectionObject::to_collection_data)
            .collect();

        // Save state to file
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

// Trait shared by all adwaita application windows
impl AdwApplicationWindowImpl for Window {}
