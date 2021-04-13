// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WaylandSeat;
use glib::translate::ToGlibPtr;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::sys::client::wl_proxy;
use wayland_client::Proxy;

impl WaylandSeat {
    #[doc(alias = "gdk_wayland_seat_get_wl_seat")]
    pub fn wl_seat(&self) -> WlSeat {
        unsafe {
            let ptr = ffi::gdk_wayland_seat_get_wl_seat(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }
}
