use std::cell::Cell;

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
        self.instance()
            .bind_property("number", &*self.instance(), "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// ANCHOR: button_impl
// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        let incremented_number = self.number.get() + 1;
        self.instance().set_property("number", &incremented_number);
    }
}
// ANCHOR_END: button_impl
