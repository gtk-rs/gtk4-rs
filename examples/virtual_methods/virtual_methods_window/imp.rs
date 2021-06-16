use crate::base_button::*;
use crate::derived_button::DerivedButton;
use gtk::{
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

#[derive(Debug, CompositeTemplate, Default)]
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

    fn class_init(klass: &mut Self::Class) {
        BaseButton::static_type();
        DerivedButton::static_type();
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
            ctx.spawn_local(clone!(@weak b => async move {
                b.async_method().await.unwrap();
            }));
        });
        self.derived_button.connect_clicked(|b| {
            let ctx = glib::MainContext::default();
            ctx.spawn_local(clone!(@weak b => async move {
                b.async_method().await.unwrap();
            }));
        });
    }
}
impl WidgetImpl for VirtualMethodsAppWindow {}
impl WindowImpl for VirtualMethodsAppWindow {}
impl ApplicationWindowImpl for VirtualMethodsAppWindow {}
