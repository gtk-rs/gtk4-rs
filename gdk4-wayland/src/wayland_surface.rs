// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use glib::translate::*;
#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use wayland_client::{backend::ObjectId, protocol::wl_surface::WlSurface, Proxy};

#[cfg(feature = "wayland_crate")]
use crate::ffi;
use crate::{prelude::*, WaylandSurface};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::WaylandSurface>> Sealed for T {}
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of
/// [`WaylandSurface`](crate::WaylandSurface).
pub trait WaylandSurfaceExtManual: sealed::Sealed + IsA<WaylandSurface> + 'static {
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
        unsafe {
            let surface_ptr =
                ffi::gdk_wayland_surface_get_wl_surface(self.as_ref().to_glib_none().0);
            if surface_ptr.is_null() {
                None
            } else {
                let cnx = display.connection();
                let id = ObjectId::from_ptr(WlSurface::interface(), surface_ptr as *mut _).unwrap();

                WlSurface::from_id(&cnx, id).ok()
            }
        }
    }
}

impl<O: IsA<WaylandSurface>> WaylandSurfaceExtManual for O {}
