// Take a look at the license at the top of the repository in the LICENSE file.

use crate::X11Screen;
use glib::translate::ToGlibPtr;
use x11::xlib;

impl X11Screen {
    #[doc(alias = "gdk_x11_screen_get_xscreen")]
    #[doc(alias = "get_xscreen")]
    pub unsafe fn xscreen(&self) -> *mut xlib::Screen {
        ffi::gdk_x11_screen_get_xscreen(self.to_glib_none().0)
    }
}
