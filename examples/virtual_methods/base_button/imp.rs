use super::BaseButtonExt;
use gtk::{
    gio,
    glib::{self, translate::from_glib_borrow, Error},
    prelude::*,
    subclass::prelude::*,
};
use std::{future::Future, pin::Pin};

pub type BaseButtonInstance = <BaseButton as super::ObjectSubclass>::Instance;
pub type PinnedFuture = Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>>;

/// GObject glue code for our BaseButtonClass which holds the function pointers to our virtual functions.
#[repr(C)]
pub struct BaseButtonClass {
    pub parent_class: gtk::ffi::GtkButtonClass,
    pub sync_method: Option<unsafe extern "C" fn(*mut BaseButtonInstance)>,
    pub async_method: Option<unsafe extern "C" fn(*mut BaseButtonInstance) -> PinnedFuture>,
}

unsafe impl ClassStruct for BaseButtonClass {
    type Type = super::imp::BaseButton;
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

#[derive(Debug)]
pub struct BaseButton {}

// Virtual method default implementation trampolines
unsafe extern "C" fn sync_method_default_trampoline(this: *mut BaseButtonInstance) {
    let imp = (*this).impl_();
    imp.sync_method(&from_glib_borrow(this))
}

unsafe extern "C" fn async_method_default_trampoline(
    this: *mut BaseButtonInstance,
) -> PinnedFuture {
    let imp = (*this).impl_();
    imp.async_method(&from_glib_borrow(this))
}

#[no_mangle]
pub unsafe extern "C" fn BaseButton_sync_method(this: *mut BaseButtonInstance) {
    let klass = glib::subclass::types::InstanceStruct::class(&*this);

    (klass.sync_method.unwrap())(this)
}

#[no_mangle]
pub unsafe extern "C" fn BaseButton_async_method(this: *mut BaseButtonInstance) -> PinnedFuture {
    let klass = glib::subclass::types::InstanceStruct::class(&*this);
    klass.async_method.unwrap()(this)
}

/// Default implementations of our sync_method and async_method.
impl BaseButton {
    fn sync_method(&self, obj: &super::BaseButton) {
        obj.set_label("BaseButton sync");
    }

    fn async_method(&self, obj: &super::BaseButton) -> PinnedFuture {
        let cloned_obj = obj.clone();
        Box::pin(gio::GioFuture::new(obj, move |_, _, send| {
            cloned_obj.set_label("BaseButton async");
            send.resolve(Ok(()));
        }))
    }
}

#[glib::object_subclass]
impl ObjectSubclass for BaseButton {
    const NAME: &'static str = "ExampleBaseButton";
    type ParentType = gtk::Button;
    type Type = super::BaseButton;
    type Class = BaseButtonClass;

    fn new() -> Self {
        Self {}
    }

    fn class_init(klass: &mut Self::Class) {
        klass.sync_method = Some(sync_method_default_trampoline);
        klass.async_method = Some(async_method_default_trampoline);
    }
}

impl ObjectImpl for BaseButton {
    fn constructed(&self, obj: &Self::Type) {
        // For demo purposes, call the sync_method during construction to set the button label
        obj.sync_method();
    }
}

impl WidgetImpl for BaseButton {}
impl ButtonImpl for BaseButton {}
