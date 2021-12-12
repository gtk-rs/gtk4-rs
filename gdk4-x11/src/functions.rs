// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use crate::X11Display;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use glib::translate::*;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
use x11::xlib;

#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
#[doc(alias = "gdk_x11_get_xatom_by_name_for_display")]
pub fn x11_get_xatom_by_name_for_display(display: &X11Display, atom_name: &str) -> xlib::Atom {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_x11_get_xatom_by_name_for_display(
            display.to_glib_none().0,
            atom_name.to_glib_none().0,
        )
    }
}

#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
#[doc(alias = "gdk_x11_get_xatom_name_for_display")]
pub fn x11_get_xatom_name_for_display(
    display: &X11Display,
    xatom: xlib::Atom,
) -> Option<glib::GString> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gdk_x11_get_xatom_name_for_display(
            display.to_glib_none().0,
            xatom,
        ))
    }
}
