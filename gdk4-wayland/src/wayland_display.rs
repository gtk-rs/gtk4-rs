// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WaylandDisplay;
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use glib::translate::ToGlibPtr;
#[cfg(any(all(feature = "v4_4", feature = "egl"), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(all(feature = "v4_4", feature = "egl"))))]
use khronos_egl as egl;

#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use wayland_client::{
    protocol::{wl_compositor::WlCompositor, wl_display::WlDisplay},
    sys::client::wl_proxy,
    Proxy,
};

impl WaylandDisplay {
    #[cfg(any(all(feature = "v4_4", feature = "egl"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(all(feature = "v4_4", feature = "egl"))))]
    #[doc(alias = "gdk_wayland_display_get_egl_display")]
    #[doc(alias = "get_egl_display")]
    pub fn egl_display(&self) -> Option<egl::Display> {
        unsafe {
            let ptr = ffi::gdk_wayland_display_get_egl_display(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(egl::Display::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "gdk_wayland_display_get_wl_compositor")]
    #[doc(alias = "get_wl_compositor")]
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    pub fn wl_compositor(&self) -> WlCompositor {
        unsafe {
            let ptr = ffi::gdk_wayland_display_get_wl_compositor(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }

    #[doc(alias = "gdk_wayland_display_get_wl_display")]
    #[doc(alias = "get_wl_display")]
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    pub fn wl_display(&self) -> WlDisplay {
        unsafe {
            let ptr = ffi::gdk_wayland_display_get_wl_display(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }
}
