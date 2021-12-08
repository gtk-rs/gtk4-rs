// Take a look at the license at the top of the repository in the LICENSE file.

use crate::X11Display;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ObjectType;
#[cfg(any(feature = "v4_4", feature = "dox"))]
use khronos_egl as egl;
use std::boxed::Box as Box_;
use std::mem::transmute;
use x11::xlib;

impl X11Display {
    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
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

    #[doc(alias = "gdk_x11_display_get_xdisplay")]
    #[doc(alias = "get_xdisplay")]
    pub unsafe fn xdisplay(&self) -> *mut xlib::Display {
        ffi::gdk_x11_display_get_xdisplay(self.to_glib_none().0)
    }

    #[doc(alias = "gdk_x11_display_get_xscreen")]
    #[doc(alias = "get_xscreen")]
    pub unsafe fn xscreen(&self) -> *mut xlib::Screen {
        ffi::gdk_x11_display_get_xscreen(self.to_glib_none().0)
    }

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
}
