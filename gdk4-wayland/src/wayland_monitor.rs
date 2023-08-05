// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WaylandMonitor;
#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use {crate::prelude::*, glib::translate::*};

#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use wayland_client::{backend::ObjectId, protocol::wl_output::WlOutput, Proxy};

impl WaylandMonitor {
    #[doc(alias = "gdk_wayland_monitor_get_wl_output")]
    #[doc(alias = "get_wl_output")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    pub fn wl_output(&self) -> Option<WlOutput> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        unsafe {
            let output_ptr = ffi::gdk_wayland_monitor_get_wl_output(self.to_glib_none().0);
            if output_ptr.is_null() {
                None
            } else {
                let cnx = display.connection();
                let id = ObjectId::from_ptr(WlOutput::interface(), output_ptr as *mut _).unwrap();

                WlOutput::from_id(&cnx, id).ok()
            }
        }
    }
}
