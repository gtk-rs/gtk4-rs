// Take a look at the license at the top of the repository in the LICENSE file.

use crate::X11Display;
use glib::Cast;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use glib::{signal::connect_raw, signal::SignalHandlerId, ObjectType};
#[cfg(any(all(feature = "v4_4", feature = "egl"), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(all(feature = "v4_4", feature = "egl"))))]
use khronos_egl as egl;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use std::boxed::Box as Box_;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use std::mem::transmute;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use x11::xlib;

#[cfg(not(feature = "xlib"))]
use crate::{XCursor, XWindow};
use glib::translate::*;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use x11::xlib::{Cursor as XCursor, Window as XWindow};

impl X11Display {
    #[cfg(any(all(feature = "v4_4", feature = "egl"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(all(feature = "v4_4", feature = "egl"))))]
    #[doc(alias = "gdk_x11_display_get_egl_display")]
    #[doc(alias = "get_egl_display")]
    pub fn egl_display(&self) -> Option<egl::Display> {
        unsafe {
            let ptr = ffi::gdk_x11_display_get_egl_display(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(egl::Display::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "gdk_x11_display_get_xcursor")]
    #[doc(alias = "get_xcursor")]
    pub fn xcursor(&self, cursor: &gdk::Cursor) -> XCursor {
        unsafe { ffi::gdk_x11_display_get_xcursor(self.to_glib_none().0, cursor.to_glib_none().0) }
    }

    #[doc(alias = "gdk_x11_display_get_xrootwindow")]
    #[doc(alias = "get_xrootwindow")]
    pub fn xrootwindow(&self) -> XWindow {
        unsafe { ffi::gdk_x11_display_get_xrootwindow(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "xlib", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
    #[doc(alias = "gdk_x11_display_get_xdisplay")]
    #[doc(alias = "get_xdisplay")]
    pub unsafe fn xdisplay(&self) -> *mut xlib::Display {
        ffi::gdk_x11_display_get_xdisplay(self.to_glib_none().0) as *mut xlib::Display
    }

    #[cfg(any(feature = "xlib", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
    #[doc(alias = "gdk_x11_display_get_xscreen")]
    #[doc(alias = "get_xscreen")]
    pub unsafe fn xscreen(&self) -> *mut xlib::Screen {
        ffi::gdk_x11_display_get_xscreen(self.to_glib_none().0) as *mut xlib::Screen
    }

    #[cfg(any(feature = "xlib", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
    #[doc(alias = "xevent")]
    pub unsafe fn connect_xevent<F: Fn(&Self, *mut xlib::XEvent) -> glib::Continue + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn xevent_trampoline<
            F: Fn(&X11Display, *mut xlib::XEvent) -> glib::Continue + 'static,
        >(
            this: *mut ffi::GdkX11Display,
            xevent: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), xevent as *mut xlib::XEvent).into_glib()
        }
        let f: Box_<F> = Box_::new(f);
        connect_raw(
            self.as_ptr() as *mut _,
            b"xevent\0".as_ptr() as *const _,
            Some(transmute::<_, unsafe extern "C" fn()>(
                xevent_trampoline::<F> as *const (),
            )),
            Box_::into_raw(f),
        )
    }

    #[doc(alias = "gdk_x11_display_set_program_class")]
    pub fn set_program_class(&self, program_class: &str) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_x11_display_set_program_class(
                self.upcast_ref::<gdk::Display>().to_glib_none().0,
                program_class.to_glib_none().0,
            );
        }
    }
}
