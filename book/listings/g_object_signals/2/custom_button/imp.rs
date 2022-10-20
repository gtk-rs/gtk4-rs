use std::cell::Cell;

use glib::subclass::Signal;
use glib::{BindingFlags, ParamSpec, ParamSpecInt, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

// Object holding the state
#[derive(Default)]
pub struct CustomButton {
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
impl ObjectImpl for CustomButton {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("max-number-reached")
                .param_types([i32::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }
    // ANCHOR_END: object_impl

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecInt::builder("number").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "number" => {
                let input_number =
                    value.get().expect("The value needs to be of type `i32`.");
                self.number.replace(input_number);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "number" => self.number.get().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let instance = self.instance();
        instance
            .bind_property("number", &*instance, "label")
            .flags(BindingFlags::SYNC_CREATE)
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
        let incremented_number = self.number.get() + 1;
        let instance = self.instance();
        // If `number` reached `MAX_NUMBER`,
        // emit "max-number-reached" signal and set `number` back to 0
        if incremented_number == MAX_NUMBER {
            instance.emit_by_name::<()>("max-number-reached", &[&incremented_number]);
            instance.set_property("number", &0);
        } else {
            instance.set_property("number", &incremented_number);
        }
    }
}
// ANCHOR_END: button_impl
