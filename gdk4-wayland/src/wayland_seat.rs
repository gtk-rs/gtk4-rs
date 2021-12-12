// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WaylandSeat;
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use glib::translate::ToGlibPtr;
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use wayland_client::{protocol::wl_seat::WlSeat, sys::client::wl_proxy, Proxy};

impl WaylandSeat {
    #[doc(alias = "gdk_wayland_seat_get_wl_seat")]
    #[doc(alias = "get_wl_seat")]
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    pub fn wl_seat(&self) -> WlSeat {
        unsafe {
            let ptr = ffi::gdk_wayland_seat_get_wl_seat(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }
}
