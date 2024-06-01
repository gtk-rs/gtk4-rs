// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
#[cfg(feature = "xlib")]
#[cfg_attr(docsrs, doc(cfg(feature = "xlib")))]
use x11::xlib::Window as XWindow;

#[cfg(not(feature = "xlib"))]
use crate::XWindow;
use crate::{ffi, X11Surface};

impl X11Surface {
    #[doc(alias = "gdk_x11_surface_get_xid")]
    #[doc(alias = "get_xid")]
    pub fn xid(&self) -> XWindow {
        unsafe { ffi::gdk_x11_surface_get_xid(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_x11_surface_lookup_for_display")]
    pub fn lookup_for_display(display: &crate::X11Display, window: XWindow) -> Option<X11Surface> {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gdk_x11_surface_lookup_for_display(
                display.to_glib_none().0,
                window,
            ))
        }
    }
}
