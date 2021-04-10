use glib::subclass::Signal;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use once_cell::sync::Lazy;
use std::cell::RefCell;

// Object holding the state
#[derive(Default)]
pub struct CustomButton {
    number: RefCell<i32>,
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
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.set_label(&self.number.borrow().to_string());
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder(
                // Signal name
                "max-number-reached",
                // Types of the values which will be sent to the receiver
                &[i32::static_type().into()],
                // Type of the value the receiver sends back
                <()>::static_type().into(),
            )
            .build()]
        });
        SIGNALS.as_ref()
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// ANCHOR: button_impl
static MAX_NUMBER: i32 = 8;
// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self, button: &Self::Type) {
        // Increase `number` by 1
        *self.number.borrow_mut() += 1;
        // If `number` reached `MAX_NUMBER`,
        // emit "max-number-reached" signal and set `number` back to 0
        if *self.number.borrow() == MAX_NUMBER {
            button
                .emit_by_name("max-number-reached", &[&*self.number.borrow()])
                .unwrap();
            self.number.replace(0);
        }
        // Set label of `button`
        button.set_label(&self.number.borrow().to_string())
    }
}
// ANCHOR_END: button_impl
