use std::cell::RefCell;
use std::sync::OnceLock;

use glib::Properties;
use gtk::gdk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{AccessibleRole, GestureClick, Label, accessible, glib};

// ANCHOR: subclass
#[derive(Properties, Default)]
#[properties(wrapper_type = super::CustomButton)]
pub struct CustomButton {
    #[property(get, set)]
    label: RefCell<String>,
    child: RefCell<Option<Label>>,
}

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
        klass.add_binding_signal(
            gdk::Key::space,
            gdk::ModifierType::empty(),
            "activate",
        );
        klass.add_binding_signal(
            gdk::Key::KP_Enter,
            gdk::ModifierType::empty(),
            "activate",
        );
        klass.add_binding_signal(
            gdk::Key::Return,
            gdk::ModifierType::empty(),
            "activate",
        );
    }
}
// ANCHOR_END: subclass

// ANCHOR: object_impl
#[glib::derived_properties]
impl ObjectImpl for CustomButton {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![glib::subclass::Signal::builder("activate").action().build()]
        })
    }

    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        // Make the widget focusable so keyboard users can reach it
        obj.set_focusable(true);
        // Also allow focusing by clicking
        obj.set_focus_on_click(true);

        // Create a child label and bind its text to our "label" property
        let child = Label::new(None);
        child.set_parent(&*obj);
        obj.update_relation(&[accessible::Relation::LabelledBy(&[child.upcast_ref()])]);
        obj.bind_property("label", &child, "label")
            .sync_create()
            .build();
        self.child.replace(Some(child));

        // Handle click events
        let gesture = GestureClick::new();
        let button = obj.downgrade();
        gesture.connect_released(move |_, _, _, _| {
            if let Some(button) = button.upgrade() {
                button.emit_by_name::<()>("activate", &[]);
            }
        });
        obj.add_controller(gesture);

        // Add an activation handler
        obj.connect_local("activate", false, move |values| {
            let button = values[0].get::<super::CustomButton>().unwrap();
            println!("Button '{}' activated!", button.label());
            None
        });
    }

    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for CustomButton {}
// ANCHOR_END: object_impl
