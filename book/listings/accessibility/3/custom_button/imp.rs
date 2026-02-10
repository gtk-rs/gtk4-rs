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
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}
// ANCHOR_END: subclass

// ANCHOR: object_impl
impl ObjectImpl for CustomButton {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        // Make the widget focusable so keyboard users can reach it
        obj.set_focusable(true);
        // Also allow focusing by clicking
        obj.set_focus_on_click(true);
        obj.set_css_classes(&["card", "activatable"]);

        // Handle click events
        let gesture = GestureClick::new();
        gesture.connect_released(|_, _, _, _| {
            println!("Custom button clicked!");
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
