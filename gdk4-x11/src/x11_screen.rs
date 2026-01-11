// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
#[cfg(feature = "xlib")]
#[cfg_attr(docsrs, doc(cfg(feature = "xlib")))]
use x11::xlib::{self, XID};

#[cfg(not(feature = "xlib"))]
use crate::XID;
use crate::{X11Screen, ffi};

impl X11Screen {
    #[cfg(feature = "xlib")]
    #[cfg_attr(docsrs, doc(cfg(feature = "xlib")))]
    #[doc(alias = "gdk_x11_screen_get_xscreen")]
    #[doc(alias = "get_xscreen")]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn xscreen(&self) -> *mut xlib::Screen {
        unsafe { ffi::gdk_x11_screen_get_xscreen(self.to_glib_none().0) as *mut xlib::Screen }
    }

    #[doc(alias = "gdk_x11_screen_get_monitor_output")]
    #[doc(alias = "get_monitor_output")]
    pub fn monitor_output(&self, monitor_num: i32) -> XID {
        unsafe { ffi::gdk_x11_screen_get_monitor_output(self.to_glib_none().0, monitor_num) }
    }
}
