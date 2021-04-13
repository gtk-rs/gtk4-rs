// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WaylandSurface;
use glib::translate::ToGlibPtr;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::sys::client::wl_proxy;
use wayland_client::Proxy;

impl WaylandSurface {
    #[doc(alias = "gdk_wayland_surface_get_wl_surface")]
    pub fn wl_surface(&self) -> WlSurface {
        unsafe {
            let ptr = ffi::gdk_wayland_surface_get_wl_surface(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }
}
