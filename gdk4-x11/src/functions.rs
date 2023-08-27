// Take a look at the license at the top of the repository in the LICENSE file.

pub use crate::auto::functions::*;
use crate::X11Display;
use glib::{translate::*, IntoGStr};

#[cfg(not(feature = "xlib"))]
use crate::XAtom;
#[cfg(feature = "xlib")]
#[cfg_attr(docsrs, doc(cfg(feature = "xlib")))]
use x11::xlib::Atom as XAtom;

#[doc(alias = "gdk_x11_get_xatom_by_name_for_display")]
pub fn x11_get_xatom_by_name_for_display(display: &X11Display, atom_name: impl IntoGStr) -> XAtom {
    skip_assert_initialized!();
    unsafe {
        atom_name.run_with_gstr(|atom_name| {
            ffi::gdk_x11_get_xatom_by_name_for_display(display.to_glib_none().0, atom_name.as_ptr())
        })
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
