// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ShortcutAction, ShortcutTrigger};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkShortcut")]
    pub struct Shortcut(Object<ffi::GtkShortcut, ffi::GtkShortcutClass>);

    match fn {
        type_ => || ffi::gtk_shortcut_get_type(),
    }
}

impl Shortcut {
    #[doc(alias = "gtk_shortcut_new")]
    pub fn new(
        trigger: Option<impl IsA<ShortcutTrigger>>,
        action: Option<impl IsA<ShortcutAction>>,
    ) -> Shortcut {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_shortcut_new(
                trigger.map(|p| p.upcast()).into_glib_ptr(),
                action.map(|p| p.upcast()).into_glib_ptr(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Shortcut`] objects.
    ///
    /// This method returns an instance of [`ShortcutBuilder`](crate::builders::ShortcutBuilder) which can be used to create [`Shortcut`] objects.
    pub fn builder() -> ShortcutBuilder {
        ShortcutBuilder::new()
    }

    #[doc(alias = "gtk_shortcut_get_action")]
    #[doc(alias = "get_action")]
    pub fn action(&self) -> Option<ShortcutAction> {
        unsafe { from_glib_none(ffi::gtk_shortcut_get_action(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_shortcut_get_arguments")]
    #[doc(alias = "get_arguments")]
    pub fn arguments(&self) -> Option<glib::Variant> {
        unsafe { from_glib_none(ffi::gtk_shortcut_get_arguments(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_shortcut_get_trigger")]
    #[doc(alias = "get_trigger")]
    pub fn trigger(&self) -> Option<ShortcutTrigger> {
        unsafe { from_glib_none(ffi::gtk_shortcut_get_trigger(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_shortcut_set_action")]
    #[doc(alias = "action")]
    pub fn set_action(&self, action: Option<impl IsA<ShortcutAction>>) {
        unsafe {
            ffi::gtk_shortcut_set_action(
                self.to_glib_none().0,
                action.map(|p| p.upcast()).into_glib_ptr(),
            );
        }
    }

    #[doc(alias = "gtk_shortcut_set_arguments")]
    #[doc(alias = "arguments")]
    pub fn set_arguments(&self, args: Option<&glib::Variant>) {
        unsafe {
            ffi::gtk_shortcut_set_arguments(self.to_glib_none().0, args.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_shortcut_set_trigger")]
    #[doc(alias = "trigger")]
    pub fn set_trigger(&self, trigger: Option<impl IsA<ShortcutTrigger>>) {
        unsafe {
            ffi::gtk_shortcut_set_trigger(
                self.to_glib_none().0,
                trigger.map(|p| p.upcast()).into_glib_ptr(),
            );
        }
    }

    #[doc(alias = "action")]
    pub fn connect_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_trampoline<F: Fn(&Shortcut) + 'static>(
            this: *mut ffi::GtkShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::action".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_action_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "arguments")]
    pub fn connect_arguments_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_arguments_trampoline<F: Fn(&Shortcut) + 'static>(
            this: *mut ffi::GtkShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::arguments".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_arguments_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "trigger")]
    pub fn connect_trigger_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_trigger_trampoline<F: Fn(&Shortcut) + 'static>(
            this: *mut ffi::GtkShortcut,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::trigger".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_trigger_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Shortcut {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Shortcut`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ShortcutBuilder {
    builder: glib::object::ObjectBuilder<'static, Shortcut>,
}

impl ShortcutBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn action(self, action: &impl IsA<ShortcutAction>) -> Self {
        Self {
            builder: self.builder.property("action", action.clone().upcast()),
        }
    }

    pub fn arguments(self, arguments: &glib::Variant) -> Self {
        Self {
            builder: self.builder.property("arguments", arguments.clone()),
        }
    }

    pub fn trigger(self, trigger: &impl IsA<ShortcutTrigger>) -> Self {
        Self {
            builder: self.builder.property("trigger", trigger.clone().upcast()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Shortcut`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Shortcut {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
