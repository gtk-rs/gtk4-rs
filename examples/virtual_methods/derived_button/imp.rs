use crate::base_button::*;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};

#[derive(Debug, Default)]
pub struct DerivedButton {}

#[glib::object_subclass]
impl ObjectSubclass for DerivedButton {
    const NAME: &'static str = "ExampleDerivedButton";
    type ParentType = BaseButton;
    type Type = super::DerivedButton;
}

impl ObjectImpl for DerivedButton {}
impl WidgetImpl for DerivedButton {}
impl ButtonImpl for DerivedButton {}

/// Implement the base trait and override the functions
impl BaseButtonImpl for DerivedButton {
    fn sync_method(&self, obj: &BaseButton, extra_text: Option<String>) {
        if let Some(text) = extra_text {
            obj.set_label(&format!("DerivedButton sync {text}"));
        } else {
            obj.set_label("DerivedButton sync");
        }
    }

    fn async_method(&self, obj: &BaseButton) -> PinnedFuture {
        let obj_cloned = obj.clone();
        Box::pin(gio::GioFuture::new(obj, move |_, _, send| {
            obj_cloned.set_label("DerivedButton async");
            send.resolve(Ok(()));
        }))
    }
}
