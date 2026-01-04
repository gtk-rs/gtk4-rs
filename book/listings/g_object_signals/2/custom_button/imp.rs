use std::cell::Cell;
use std::sync::OnceLock;

use glib::Properties;
use glib::subclass::Signal;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::CustomButton)]
pub struct CustomButton {
    #[property(get, set)]
    number: Cell<i32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// ANCHOR: object_impl
// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for CustomButton {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("max-number-reached")
                    .param_types([i32::static_type()])
                    .build(),
            ]
        })
    }
    // ANCHOR_END: object_impl

    fn constructed(&self) {
        self.parent_constructed();

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();
        obj.bind_property("number", obj.as_ref(), "label")
            .sync_create()
            .build();
    }
}

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// ANCHOR: button_impl
static MAX_NUMBER: i32 = 8;

// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        let incremented_number = self.obj().number() + 1;
        let obj = self.obj();
        // If `number` reached `MAX_NUMBER`,
        // emit "max-number-reached" signal and set `number` back to 0
        if incremented_number == MAX_NUMBER {
            obj.emit_by_name::<()>("max-number-reached", &[&incremented_number]);
            obj.set_number(0);
        } else {
            obj.set_number(incremented_number);
        }
    }
}
// ANCHOR_END: button_impl
