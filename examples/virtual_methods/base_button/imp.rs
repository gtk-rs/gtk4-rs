use gtk::{gio, glib, prelude::*, subclass::prelude::*};

/// Implementation of `BaseButton`.
use super::{BaseButtonExt, PinnedFuture};

#[derive(Debug, Default)]
pub struct BaseButton;

impl BaseButton {
    /// Implementation for non-virtual methods.
    pub(super) fn non_virtual_method(&self) {
        let obj = self.obj();
        obj.set_label("Non-virtual method called");
    }

    /// Default implementations virtual methods.
    fn sync_method_default(&self, extra_text: Option<&str>) {
        let obj = self.obj();
        if let Some(text) = extra_text {
            obj.set_label(&format!("BaseButton sync: {text}"));
        } else {
            obj.set_label("BaseButton sync");
        }
    }

    fn async_method_default(&self) -> PinnedFuture<Result<(), glib::Error>> {
        let obj = self.obj();
        Box::pin(gio::GioFuture::new(&*obj, |obj, _, send| {
            obj.set_label("BaseButton async");
            send.resolve(Ok(()));
        }))
    }
}

#[glib::object_subclass]
impl ObjectSubclass for BaseButton {
    const NAME: &'static str = "ExampleBaseButton";
    type ParentType = gtk::Button;
    type Type = super::BaseButton;
    type Class = super::Class;

    /// Initialize the class struct with the default implementations of the
    /// virtual methods.
    fn class_init(klass: &mut Self::Class) {
        klass.sync_method = |obj, extra_text| {
            obj.imp().sync_method_default(extra_text);
        };
        klass.async_method = |obj| obj.imp().async_method_default();
    }
}

impl ObjectImpl for BaseButton {
    fn constructed(&self) {
        self.parent_constructed();

        // For demo purposes, call the `non_virtual_method()` during construction to set
        // the button label
        self.obj().non_virtual_method();
    }
}

impl WidgetImpl for BaseButton {}
impl ButtonImpl for BaseButton {}
