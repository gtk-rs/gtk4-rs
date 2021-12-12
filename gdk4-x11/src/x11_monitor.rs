// Take a look at the license at the top of the repository in the LICENSE file.

use crate::X11Monitor;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use glib::translate::*;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use x11::xlib;

impl X11Monitor {
    #[cfg(any(feature = "xlib", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
    #[doc(alias = "gdk_x11_monitor_get_output")]
    #[doc(alias = "get_output")]
    pub fn output(&self) -> xlib::XID {
        unsafe { ffi::gdk_x11_monitor_get_output(self.to_glib_none().0) }
    }
}
