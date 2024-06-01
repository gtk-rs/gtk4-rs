// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
#[cfg(feature = "xlib")]
#[cfg_attr(docsrs, doc(cfg(feature = "xlib")))]
use x11::xlib::XID;

#[cfg(not(feature = "xlib"))]
use crate::XID;
use crate::{ffi, X11Monitor};

impl X11Monitor {
    #[doc(alias = "gdk_x11_monitor_get_output")]
    #[doc(alias = "get_output")]
    pub fn output(&self) -> XID {
        unsafe { ffi::gdk_x11_monitor_get_output(self.to_glib_none().0) }
    }
}
