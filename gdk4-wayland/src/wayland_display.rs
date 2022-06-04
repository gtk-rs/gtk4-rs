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
    backend::ObjectId,
    protocol::{wl_compositor::WlCompositor, wl_display::WlDisplay},
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
    pub fn wl_compositor(&self) -> Option<WlCompositor> {
        unsafe {
            let display_ptr = ffi::gdk_wayland_display_get_wl_display(self.to_glib_none().0);
            let compositor_ptr = ffi::gdk_wayland_display_get_wl_compositor(self.to_glib_none().0);
            if compositor_ptr.is_null() {
                None
            } else {
                let backend = wayland_backend::sys::client::Backend::from_foreign_display(
                    display_ptr as *mut _,
                );
                let cnx = wayland_client::Connection::from_backend(backend);
                let compositor_id =
                    ObjectId::from_ptr(&WlCompositor::interface(), compositor_ptr as *mut _)
                        .unwrap();

                WlCompositor::from_id(&cnx, compositor_id).ok()
            }
        }
    }

    #[doc(alias = "gdk_wayland_display_get_wl_display")]
    #[doc(alias = "get_wl_display")]
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    pub fn wl_display(&self) -> WlDisplay {
        unsafe {
            let display_ptr = ffi::gdk_wayland_display_get_wl_display(self.to_glib_none().0);
            let backend =
                wayland_backend::sys::client::Backend::from_foreign_display(display_ptr as *mut _);
            let cnx = wayland_client::Connection::from_backend(backend);
            let display_id =
                ObjectId::from_ptr(&WlDisplay::interface(), display_ptr as *mut _).unwrap();

            WlDisplay::from_id(&cnx, display_id).unwrap()
        }
    }
}
