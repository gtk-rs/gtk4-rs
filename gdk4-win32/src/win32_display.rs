// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Win32Display;

#[cfg(any(all(feature = "v4_4", feature = "egl"), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(all(feature = "v4_4", feature = "egl"))))]
use glib::translate::ToGlibPtr;
#[cfg(any(all(feature = "v4_4", feature = "egl"), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(all(feature = "v4_4", feature = "egl"))))]
use khronos_egl as egl;

impl Win32Display {
    #[cfg(any(all(feature = "v4_4", feature = "egl"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(all(feature = "v4_4", feature = "egl"))))]
    #[doc(alias = "gdk_win32_display_get_egl_display")]
    #[doc(alias = "get_egl_display")]
    pub fn egl_display(&self) -> Option<egl::Display> {
        unsafe {
            let ptr = ffi::gdk_win32_display_get_egl_display(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(egl::Display::from_ptr(ptr))
            }
        }
    }
}
