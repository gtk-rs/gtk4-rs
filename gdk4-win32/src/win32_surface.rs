// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, prelude::*, Win32Surface, HWND};

impl Win32Surface {
    #[doc(alias = "gdk_win32_surface_lookup_for_display")]
    pub fn lookup_for_display(
        display: &impl IsA<gdk::Display>,
        anid: HWND,
    ) -> Option<gdk::Surface> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gdk_win32_surface_lookup_for_display(
                display.as_ref().to_glib_none().0,
                anid.0,
            ))
        }
    }

    #[doc(alias = "gdk_win32_surface_get_handle")]
    #[doc(alias = "get_handle")]
    pub fn handle(&self) -> HWND {
        HWND(unsafe { ffi::gdk_win32_surface_get_handle(self.to_glib_none().0) })
    }

    #[cfg_attr(feature = "v4_8", deprecated = "Since 4.8")]
    #[doc(alias = "gdk_win32_surface_get_impl_hwnd")]
    #[doc(alias = "get_impl_hwnd")]
    pub fn impl_hwnd(surface: &impl IsA<gdk::Surface>) -> HWND {
        assert_initialized_main_thread!();
        HWND(unsafe { ffi::gdk_win32_surface_get_impl_hwnd(surface.as_ref().to_glib_none().0) })
    }
}
