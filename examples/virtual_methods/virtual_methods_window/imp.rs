use crate::base_button::*;
use gtk::{glib, prelude::*, subclass::prelude::*, CompositeTemplate};

#[derive(Debug, CompositeTemplate)]
#[template(file = "window.ui")]
pub struct VirtualMethodsAppWindow {
    #[template_child]
    base_button: TemplateChild<BaseButton>,
    #[template_child]
    derived_button: TemplateChild<BaseButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for VirtualMethodsAppWindow {
    const NAME: &'static str = "ExampleVirtualMethodsAppWindow";
    type ParentType = gtk::ApplicationWindow;
    type Type = super::VirtualMethodsAppWindow;

    fn new() -> Self {
        Self {
            base_button: TemplateChild::default(),
            derived_button: TemplateChild::default(),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for VirtualMethodsAppWindow {
    fn constructed(&self, _obj: &Self::Type) {
        self.base_button.connect_clicked(|b| {
            let ctx = glib::MainContext::default();
            let b = b.clone();
            ctx.spawn_local(async move {
                b.async_method().await.unwrap();
            });
        });
        self.derived_button.connect_clicked(|b| {
            let ctx = glib::MainContext::default();
            let b = b.clone();
            ctx.spawn_local(async move {
                b.async_method().await.unwrap();
            });
        });
    }
}
impl WidgetImpl for VirtualMethodsAppWindow {}
impl WindowImpl for VirtualMethodsAppWindow {}
impl ApplicationWindowImpl for VirtualMethodsAppWindow {}
