use std::sync::OnceLock;

use gtk::gdk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{AccessibleRole, GestureClick, glib};

// ANCHOR: subclass
#[derive(Default)]
pub struct CustomButton;

#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "CustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        // Set the accessible role to Button
        klass.set_accessible_role(AccessibleRole::Button);
        klass.set_css_name("custom-button");
        klass.set_layout_manager_type::<gtk::BinLayout>();

        // Bind keyboard shortcuts for activation (Enter and Space)
        klass.add_binding_signal(gdk::Key::space, gdk::ModifierType::empty(), "activate");
        klass.add_binding_signal(gdk::Key::KP_Enter, gdk::ModifierType::empty(), "activate");
        klass.add_binding_signal(gdk::Key::Return, gdk::ModifierType::empty(), "activate");
    }
}
// ANCHOR_END: subclass

// ANCHOR: object_impl
impl ObjectImpl for CustomButton {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| vec![glib::subclass::Signal::builder("activate").action().build()])
    }

    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        // Make the widget focusable so keyboard users can reach it
        obj.set_focusable(true);
        // Also allow focusing by clicking
        obj.set_focus_on_click(true);

        // Handle click events
        let gesture = GestureClick::new();
        let button = obj.downgrade();
        gesture.connect_released(move |_, _, _, _| {
            if let Some(button) = button.upgrade() {
                button.emit_by_name::<()>("activate", &[]);
            }
        });
        obj.add_controller(gesture);
    }

    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for CustomButton {}
// ANCHOR_END: object_impl
