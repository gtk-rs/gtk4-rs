// Take a look at the license at the top of the repository in the LICENSE file.

use crate::X11Monitor;
#[cfg(not(feature = "xlib"))]
use crate::XID;
use glib::translate::*;
#[cfg(any(feature = "xlib", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "xlib")))]
use x11::xlib::XID;

impl X11Monitor {
    #[doc(alias = "gdk_x11_monitor_get_output")]
    #[doc(alias = "get_output")]
    pub fn output(&self) -> XID {
        unsafe { ffi::gdk_x11_monitor_get_output(self.to_glib_none().0) }
    }
}
