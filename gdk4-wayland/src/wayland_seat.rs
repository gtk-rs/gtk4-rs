// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::c_void;
use std::ptr::NonNull;

#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use wayland_client::{Proxy, backend::ObjectId, protocol::wl_seat::WlSeat};

#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use crate::prelude::*;
use crate::{WaylandSeat, ffi};
use glib::translate::*;

impl WaylandSeat {
    #[doc(alias = "gdk_wayland_seat_get_wl_seat")]
    #[doc(alias = "get_wl_seat")]
    pub fn wl_seat_raw(&self) -> Option<NonNull<c_void>> {
        NonNull::new(unsafe { ffi::gdk_wayland_seat_get_wl_seat(self.to_glib_none().0) })
    }

    #[doc(alias = "gdk_wayland_seat_get_wl_seat")]
    #[doc(alias = "get_wl_seat")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    pub fn wl_seat(&self) -> Option<WlSeat> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        let ptr = self.wl_seat_raw()?;
        unsafe {
            let cnx = display.connection();
            let id = ObjectId::from_ptr(WlSeat::interface(), ptr.as_ptr() as *mut _).unwrap();

            WlSeat::from_id(&cnx, id).ok()
        }
    }
}
