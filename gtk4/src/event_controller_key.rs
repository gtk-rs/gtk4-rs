// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, mem::transmute};

use gdk::Key;
use glib::{signal::connect_raw, translate::*, SignalHandlerId};

use crate::{ffi, prelude::*, EventControllerKey};

impl EventControllerKey {
    pub fn connect_key_pressed<
        F: Fn(&EventControllerKey, Key, u32, gdk::ModifierType) -> glib::Propagation + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn key_pressed_trampoline<
            F: Fn(&EventControllerKey, Key, u32, gdk::ModifierType) -> glib::Propagation + 'static,
        >(
            this: *mut ffi::GtkEventControllerKey,
            keyval: libc::c_uint,
            keycode: libc::c_uint,
            state: gdk::ffi::GdkModifierType,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                from_glib(keyval),
                keycode,
                from_glib(state),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"key-pressed".as_ptr() as *const _,
                Some(transmute::<*const (), unsafe extern "C" fn()>(
                    key_pressed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_key_released<
        F: Fn(&EventControllerKey, Key, u32, gdk::ModifierType) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn key_released_trampoline<
            F: Fn(&EventControllerKey, Key, u32, gdk::ModifierType) + 'static,
        >(
            this: *mut ffi::GtkEventControllerKey,
            keyval: libc::c_uint,
            keycode: libc::c_uint,
            state: gdk::ffi::GdkModifierType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                from_glib(keyval),
                keycode,
                from_glib(state),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"key-released".as_ptr() as *const _,
                Some(transmute::<*const (), unsafe extern "C" fn()>(
                    key_released_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
