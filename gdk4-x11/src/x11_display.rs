// Take a look at the license at the top of the repository in the LICENSE file.

use crate::X11Display;
use glib::translate::ToGlibPtr;
use x11::xlib;

impl X11Display {
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
