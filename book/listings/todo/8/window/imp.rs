use std::cell::RefCell;
use std::fs::File;

use adw::subclass::prelude::*;
use adw::{prelude::*, NavigationSplitView};
use gio::Settings;
use glib::subclass::InitializingObject;
use gtk::glib::SignalHandlerId;
use gtk::{gio, glib, CompositeTemplate, Entry, FilterListModel, ListBox, Stack};
use std::cell::OnceCell;

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
    pub split_view: TemplateChild<NavigationSplitView>,
    #[template_child]
    pub stack: TemplateChild<Stack>,
    pub collections: OnceCell<gio::ListStore>,
    pub current_collection: RefCell<Option<CollectionObject>>,
    pub current_filter_model: RefCell<Option<FilterListModel>>,
    pub tasks_changed_handler_id: RefCell<Option<SignalHandlerId>>,
}
// ANCHOR_END: struct

// ANCHOR: object_subclass
// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "TodoWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();

        // Create action to remove done tasks and add to action group "win"
        klass.install_action("win.remove-done-tasks", None, |window, _, _| {
            window.remove_done_tasks();
        });

        // Create async action to create new collection and add to action group "win"
        klass.install_action_async(
            "win.new-collection",
            None,
            |window, _, _| async move {
                window.new_collection().await;
            },
        );
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
// ANCHOR_END: object_subclass

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Setup
        let obj = self.obj();
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
    fn close_request(&self) -> glib::Propagation {
        // Store task data in vector
        let backup_data: Vec<CollectionData> = self
            .obj()
            .collections()
            .iter::<CollectionObject>()
            .filter_map(|collection_object| collection_object.ok())
            .map(|collection_object| collection_object.to_collection_data())
            .collect();

        // Save state to file
        let file = File::create(data_path()).expect("Could not create json file.");
        serde_json::to_writer(file, &backup_data)
            .expect("Could not write data to json file");

        // Pass close request on to the parent
        self.parent_close_request()
    }
}
// ANCHOR_END: window_impl

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}

// Trait shared by all adwaita application windows
impl AdwApplicationWindowImpl for Window {}
