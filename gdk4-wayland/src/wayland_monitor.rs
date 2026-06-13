// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::c_void;
use std::ptr::NonNull;

#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use wayland_client::{Proxy, backend::ObjectId, protocol::wl_output::WlOutput};

#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use crate::prelude::*;
use crate::{WaylandMonitor, ffi};
use glib::translate::*;

impl WaylandMonitor {
    #[doc(alias = "gdk_wayland_monitor_get_wl_output")]
    #[doc(alias = "get_wl_output")]
    pub fn wl_output_raw(&self) -> Option<NonNull<c_void>> {
        NonNull::new(unsafe { ffi::gdk_wayland_monitor_get_wl_output(self.to_glib_none().0) })
    }

    #[doc(alias = "gdk_wayland_monitor_get_wl_output")]
    #[doc(alias = "get_wl_output")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    pub fn wl_output(&self) -> Option<WlOutput> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        let ptr = self.wl_output_raw()?;
        unsafe {
            let cnx = display.connection();
            let id = ObjectId::from_ptr(WlOutput::interface(), ptr.as_ptr() as *mut _).unwrap();

            WlOutput::from_id(&cnx, id).ok()
        }
    }
}
