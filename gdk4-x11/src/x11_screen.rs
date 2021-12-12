// Take a look at the license at the top of the repository in the LICENSE file.

use crate::X11Screen;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use glib::translate::ToGlibPtr;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use x11::xlib;

impl X11Screen {
    #[cfg(any(feature = "xlib", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
    #[doc(alias = "gdk_x11_screen_get_xscreen")]
    #[doc(alias = "get_xscreen")]
    pub unsafe fn xscreen(&self) -> *mut xlib::Screen {
        ffi::gdk_x11_screen_get_xscreen(self.to_glib_none().0) as *mut xlib::Screen
    }

    #[cfg(any(feature = "xlib", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
    #[doc(alias = "gdk_x11_screen_get_monitor_output")]
    #[doc(alias = "get_monitor_output")]
    pub fn monitor_output(&self, monitor_num: i32) -> xlib::XID {
        unsafe { ffi::gdk_x11_screen_get_monitor_output(self.to_glib_none().0, monitor_num) }
    }
}
