// Take a look at the license at the top of the repository in the LICENSE file.

use crate::X11Surface;
#[cfg(not(feature = "xlib"))]
use crate::XWindow;
use glib::translate::*;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use x11::xlib::Window as XWindow;

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
