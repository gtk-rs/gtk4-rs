// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WaylandSurface;
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use glib::translate::*;
use glib::IsA;

#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use wayland_client::{protocol::wl_surface::WlSurface, sys::client::wl_proxy, Proxy};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`WaylandSurface`](crate::WaylandSurface).
pub trait WaylandSurfaceExtManual: 'static {
    #[doc(alias = "gdk_wayland_surface_get_wl_surface")]
    #[doc(alias = "get_wl_surface")]
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    fn wl_surface(&self) -> WlSurface;
}

impl<O: IsA<WaylandSurface>> WaylandSurfaceExtManual for O {
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    fn wl_surface(&self) -> WlSurface {
        unsafe {
            let ptr = ffi::gdk_wayland_surface_get_wl_surface(self.as_ref().to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }
}
