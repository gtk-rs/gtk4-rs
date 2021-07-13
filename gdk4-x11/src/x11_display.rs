// Take a look at the license at the top of the repository in the LICENSE file.

use crate::X11Display;
use glib::translate::ToGlibPtr;
#[cfg(any(feature = "v4_4", feature = "dox"))]
use khronos_egl as egl;
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
}
