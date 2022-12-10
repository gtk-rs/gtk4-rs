// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(not(any(feature = "v4_8", feature = "dox")))]
use crate::{prelude::*, ListItem};

use crate::SignalListItemFactory;

#[cfg(not(any(feature = "v4_8", feature = "dox")))]
use glib::{
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
#[cfg(not(any(feature = "v4_8", feature = "dox")))]
use std::{boxed::Box as Box_, mem::transmute};

impl SignalListItemFactory {
    #[doc(alias = "bind")]
    #[cfg(not(any(feature = "v4_8", feature = "dox")))]
    pub fn connect_bind<F: Fn(&Self, &ListItem) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn bind_trampoline<F: Fn(&SignalListItemFactory, &ListItem) + 'static>(
            this: *mut ffi::GtkSignalListItemFactory,
            listitem: *mut ffi::GtkListItem,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(listitem))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"bind\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    bind_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "setup")]
    #[cfg(not(any(feature = "v4_8", feature = "dox")))]
    pub fn connect_setup<F: Fn(&Self, &ListItem) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn setup_trampoline<
            F: Fn(&SignalListItemFactory, &ListItem) + 'static,
        >(
            this: *mut ffi::GtkSignalListItemFactory,
            listitem: *mut ffi::GtkListItem,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(listitem))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"setup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    setup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "teardown")]
    #[cfg(not(any(feature = "v4_8", feature = "dox")))]
    pub fn connect_teardown<F: Fn(&Self, &ListItem) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn teardown_trampoline<
            F: Fn(&SignalListItemFactory, &ListItem) + 'static,
        >(
            this: *mut ffi::GtkSignalListItemFactory,
            listitem: *mut ffi::GtkListItem,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(listitem))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"teardown\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    teardown_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "unbind")]
    #[cfg(not(any(feature = "v4_8", feature = "dox")))]
    pub fn connect_unbind<F: Fn(&Self, &ListItem) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn unbind_trampoline<
            F: Fn(&SignalListItemFactory, &ListItem) + 'static,
        >(
            this: *mut ffi::GtkSignalListItemFactory,
            listitem: *mut ffi::GtkListItem,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(listitem))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unbind\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    unbind_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
