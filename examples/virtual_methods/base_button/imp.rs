use super::BaseButtonExt;
use gtk::{
    gio,
    glib::{self, Error},
    prelude::*,
    subclass::prelude::*,
};
use std::{future::Future, pin::Pin};

pub type BaseButtonInstance = super::BaseButton;
pub type PinnedFuture = Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>>;

/// GObject glue code for our BaseButtonClass which holds the function pointers to our virtual functions.
#[repr(C)]
pub struct BaseButtonClass {
    pub parent_class: gtk::ffi::GtkButtonClass,
    // If these functions are meant to be called from C, you need to make these functions
    // `unsafe extern "C" fn` & use FFI-safe types (usually raw pointers).
    pub sync_method: fn(&BaseButtonInstance, extra_text: Option<String>),
    pub async_method: fn(&BaseButtonInstance) -> PinnedFuture,
}

unsafe impl ClassStruct for BaseButtonClass {
    type Type = BaseButton;
}

#[derive(Debug, Default)]
pub struct BaseButton;

// Virtual method default implementation trampolines
fn sync_method_default_trampoline(this: &BaseButtonInstance, extra_text: Option<String>) {
    this.imp().sync_method(this, extra_text)
}

fn async_method_default_trampoline(this: &BaseButtonInstance) -> PinnedFuture {
    this.imp().async_method(this)
}

pub(super) fn base_button_sync_method(this: &BaseButtonInstance, extra_text: Option<String>) {
    let klass = this.class();
    (klass.as_ref().sync_method)(this, extra_text)
}

pub(super) fn base_button_async_method(this: &BaseButtonInstance) -> PinnedFuture {
    let klass = this.class();
    (klass.as_ref().async_method)(this)
}

/// Default implementations of our sync_method and async_method.
impl BaseButton {
    fn sync_method(&self, obj: &super::BaseButton, extra_text: Option<String>) {
        if let Some(text) = extra_text {
            obj.set_label(&format!("BaseButton sync: {text}"));
        } else {
            obj.set_label("BaseButton sync");
        }
    }

    fn async_method(&self, obj: &super::BaseButton) -> PinnedFuture {
        Box::pin(gio::GioFuture::new(
            obj,
            glib::clone!(@weak obj => move |_, _, send| {
                obj.set_label("BaseButton async");
                send.resolve(Ok(()));
            }),
        ))
    }
}

#[glib::object_subclass]
impl ObjectSubclass for BaseButton {
    const NAME: &'static str = "ExampleBaseButton";
    type ParentType = gtk::Button;
    type Type = super::BaseButton;
    type Class = BaseButtonClass;

    fn class_init(klass: &mut Self::Class) {
        klass.sync_method = sync_method_default_trampoline;
        klass.async_method = async_method_default_trampoline;
    }
}

impl ObjectImpl for BaseButton {
    fn constructed(&self) {
        self.parent_constructed();
        // For demo purposes, call the sync_method during construction to set the button label
        self.obj()
            .sync_method(Some(String::from("Sync extra text")));
    }
}

impl WidgetImpl for BaseButton {}
impl ButtonImpl for BaseButton {}
