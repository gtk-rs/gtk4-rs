// Take a look at the license at the top of the repository in the LICENSE file.

use crate::X11Display;
use glib::translate::*;

#[cfg(not(feature = "xlib"))]
use crate::XAtom;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use x11::xlib::Atom as XAtom;

#[doc(alias = "gdk_x11_get_xatom_by_name_for_display")]
pub fn x11_get_xatom_by_name_for_display(display: &X11Display, atom_name: &str) -> XAtom {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_x11_get_xatom_by_name_for_display(
            display.to_glib_none().0,
            atom_name.to_glib_none().0,
        )
    }
}

#[doc(alias = "gdk_x11_get_xatom_name_for_display")]
pub fn x11_get_xatom_name_for_display(display: &X11Display, xatom: XAtom) -> Option<glib::GString> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gdk_x11_get_xatom_name_for_display(
            display.to_glib_none().0,
            xatom,
        ))
    }
}
