// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use wayland_client::{Proxy, backend::ObjectId, protocol::wl_seat::WlSeat};
#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use {
    crate::{ffi, prelude::*},
    glib::translate::*,
};

use crate::WaylandSeat;

impl WaylandSeat {
    #[doc(alias = "gdk_wayland_seat_get_wl_seat")]
    #[doc(alias = "get_wl_seat")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    pub fn wl_seat(&self) -> Option<WlSeat> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        unsafe {
            let seat_ptr = ffi::gdk_wayland_seat_get_wl_seat(self.to_glib_none().0);
            if seat_ptr.is_null() {
                None
            } else {
                let cnx = display.connection();
                let id = ObjectId::from_ptr(WlSeat::interface(), seat_ptr as *mut _).unwrap();

                WlSeat::from_id(&cnx, id).ok()
            }
        }
    }
}
