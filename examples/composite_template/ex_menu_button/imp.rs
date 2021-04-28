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
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ExMenuButton {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        let popover = &*self.popover;
        self.toggle
            .connect_toggled(glib::clone!(@weak popover => move |toggle| {
                if toggle.is_active() {
                    popover.popup();
                }
            }));

        let toggle = &*self.toggle;
        self.popover
            .connect_closed(glib::clone!(@weak toggle => move |_| {
                toggle.set_active(false);
            }));
    }

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
