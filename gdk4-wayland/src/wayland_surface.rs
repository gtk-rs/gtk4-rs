// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WaylandSurface;
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use crate::{gdk::prelude::*, glib::translate::*};
use glib::IsA;

#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use wayland_client::{backend::ObjectId, protocol::wl_surface::WlSurface, Proxy};

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
        let display = self
            .as_ref()
            .display()
            .downcast::<crate::WaylandDisplay>()
            .unwrap();
        unsafe {
            let surface_ptr =
                ffi::gdk_wayland_surface_get_wl_surface(self.as_ref().to_glib_none().0);
            let cnx = display.connection();
            let surface_id =
                ObjectId::from_ptr(&WlSurface::interface(), surface_ptr as *mut _).unwrap();

            WlSurface::from_id(&cnx, surface_id).unwrap()
        }
    }
}
