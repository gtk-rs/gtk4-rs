use std::cell::RefCell;
use std::sync::OnceLock;

use glib::Properties;
use glib::subclass::Signal;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::TrackedButton)]
pub struct TrackedButton {
    #[property(get, set)]
    text: RefCell<String>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TrackedButton {
    const NAME: &'static str = "MyGtkAppTrackedButton";
    type Type = super::TrackedButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for TrackedButton {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("text-changed")
                    .param_types([str::static_type()])
                    .build(),
            ]
        })
    }

    fn constructed(&self) {
        self.parent_constructed();

        // Bind label to text so the visible text reflects the property.
        let obj = self.obj();
        obj.bind_property("text", obj.as_ref(), "label")
            .sync_create()
            .build();
    }
}

// Trait shared by all widgets
impl WidgetImpl for TrackedButton {}

// ANCHOR: clicked_good
// Trait shared by all buttons
impl ButtonImpl for TrackedButton {
    fn clicked(&self) {
        // GOOD: we read the value, drop the borrow, then emit.
        // If a "text-changed" handler calls back into this object - for
        // example by calling `set_text` - it can borrow `text` mutably
        // because we are no longer holding the borrow.
        let snapshot: String = self.text.borrow().clone();
        self.obj()
            .emit_by_name::<()>("text-changed", &[&snapshot]);
    }
}
// ANCHOR_END: clicked_good
