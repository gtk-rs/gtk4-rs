use crate::ffi;
use crate::WaylandSeat;
use glib::translate::ToGlibPtr;
use glib::IsA;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::sys::client::wl_proxy;
use wayland_client::Proxy;

pub trait WaylandSeatManualExt {
    fn get_wl_seat(&self) -> WlSeat;
}

impl<O: IsA<WaylandSeat>> WaylandSeatManualExt for O {
    fn get_wl_seat(&self) -> WlSeat {
        unsafe {
            let ptr = ffi::gdk_wayland_seat_get_wl_seat(self.as_ref().to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }
}
