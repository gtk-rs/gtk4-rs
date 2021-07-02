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
    pub sync_method: Option<unsafe fn(&BaseButtonInstance, extra_text: Option<String>)>,
    pub async_method: Option<unsafe fn(&BaseButtonInstance) -> PinnedFuture>,
}

unsafe impl ClassStruct for BaseButtonClass {
    type Type = BaseButton;
}

impl std::ops::Deref for BaseButtonClass {
    type Target = glib::Class<glib::Object>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const Self::Target) }
    }
}

impl std::ops::DerefMut for BaseButtonClass {
    fn deref_mut(&mut self) -> &mut glib::Class<glib::Object> {
        unsafe { &mut *(self as *mut _ as *mut glib::Class<glib::Object>) }
    }
}

#[derive(Debug, Default)]
pub struct BaseButton;

// Virtual method default implementation trampolines
unsafe fn sync_method_default_trampoline(this: &BaseButtonInstance, extra_text: Option<String>) {
    BaseButton::from_instance(this).sync_method(this, extra_text)
}

unsafe fn async_method_default_trampoline(this: &BaseButtonInstance) -> PinnedFuture {
    BaseButton::from_instance(this).async_method(this)
}

pub unsafe fn base_button_sync_method(this: &BaseButtonInstance, extra_text: Option<String>) {
    let klass = &*(this.class() as *const _ as *const BaseButtonClass);
    (klass.sync_method.unwrap())(this, extra_text)
}

pub unsafe fn base_button_async_method(this: &BaseButtonInstance) -> PinnedFuture {
    let klass = &*(this.class() as *const _ as *const BaseButtonClass);
    klass.async_method.unwrap()(this)
}

/// Default implementations of our sync_method and async_method.
impl BaseButton {
    fn sync_method(&self, obj: &super::BaseButton, extra_text: Option<String>) {
        if let Some(text) = extra_text {
            obj.set_label(&format!("BaseButton sync: {}", text));
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
        klass.sync_method = Some(sync_method_default_trampoline);
        klass.async_method = Some(async_method_default_trampoline);
    }
}

impl ObjectImpl for BaseButton {
    fn constructed(&self, obj: &Self::Type) {
        // For demo purposes, call the sync_method during construction to set the button label
        obj.sync_method(Some(String::from("Sync extra text")));
    }
}

impl WidgetImpl for BaseButton {}
impl ButtonImpl for BaseButton {}
