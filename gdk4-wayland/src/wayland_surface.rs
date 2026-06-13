// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::c_void;
use std::ptr::NonNull;

use glib::translate::*;
#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use wayland_client::{Proxy, backend::ObjectId, protocol::wl_surface::WlSurface};

use crate::ffi;
use crate::{WaylandSurface, prelude::*};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of
/// [`WaylandSurface`](crate::WaylandSurface).
pub trait WaylandSurfaceExtManual: IsA<WaylandSurface> + 'static {
    #[doc(alias = "gdk_wayland_surface_get_wl_surface")]
    #[doc(alias = "get_wl_surface")]
    fn wl_surface_raw(&self) -> Option<NonNull<c_void>> {
        NonNull::new(unsafe {
            ffi::gdk_wayland_surface_get_wl_surface(self.as_ref().to_glib_none().0)
        })
    }

    #[doc(alias = "gdk_wayland_surface_get_wl_surface")]
    #[doc(alias = "get_wl_surface")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    fn wl_surface(&self) -> Option<WlSurface> {
        let display = self
            .as_ref()
            .display()
            .downcast::<crate::WaylandDisplay>()
            .unwrap();
        let ptr = self.wl_surface_raw()?;
        unsafe {
            let cnx = display.connection();
            let id = ObjectId::from_ptr(WlSurface::interface(), ptr.as_ptr() as *mut _).unwrap();

            WlSurface::from_id(&cnx, id).ok()
        }
    }
}

impl<O: IsA<WaylandSurface>> WaylandSurfaceExtManual for O {}
