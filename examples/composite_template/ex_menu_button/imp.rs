use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "ex_menu_button.ui")]
pub struct ExMenuButton {
    #[template_child]
    pub toggle: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub popover: TemplateChild<gtk::Popover>,
}

#[glib::object_subclass]
impl ObjectSubclass for ExMenuButton {
    const NAME: &'static str = "ExMenuButton";
    type Type = super::ExMenuButton;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        Self::bind_template_callbacks(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl ExMenuButton {
    #[template_callback]
    fn toggle_toggled(&self, toggle: &gtk::ToggleButton) {
        if toggle.is_active() {
            self.popover.popup();
        }
    }
    #[template_callback(name = "popover_closed")]
    fn unset_toggle(&self) {
        self.toggle.set_active(false);
    }
}

impl ObjectImpl for ExMenuButton {
    // Needed for direct subclasses of GtkWidget;
    // Here you need to unparent all direct children
    // of your template.
    fn dispose(&self, obj: &Self::Type) {
        while let Some(child) = obj.first_child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for ExMenuButton {
    fn size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(widget, width, height, baseline);
        self.popover.present();
    }
}
