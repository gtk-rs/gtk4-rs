// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "v4_8")]
use crate::ffi;
#[cfg(not(feature = "v4_8"))]
use crate::prelude::*;
use crate::{id, MacosSurface};
#[cfg(feature = "v4_8")]
use glib::translate::*;
#[cfg(not(feature = "v4_8"))]
use std::ffi::c_void;

impl MacosSurface {
    #[doc(alias = "gdk_macos_surface_get_native_window")]
    #[doc(alias = "get_native_window")]
    pub fn native(&self) -> id {
        #[cfg(feature = "v4_8")]
        unsafe {
            let native_window_ptr = ffi::gdk_macos_surface_get_native_window(self.to_glib_none().0);
            native_window_ptr as id
        }

        #[cfg(not(feature = "v4_8"))]
        {
            let native_window_ptr: *mut c_void = ObjectExt::property(self, "native");
            native_window_ptr as id
        }
    }
}
