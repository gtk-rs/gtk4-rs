// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WaylandMonitor;
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use crate::{gdk::prelude::*, glib::translate::ToGlibPtr};
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use wayland_client::{backend::ObjectId, protocol::wl_output::WlOutput, Proxy};

impl WaylandMonitor {
    #[doc(alias = "gdk_wayland_monitor_get_wl_output")]
    #[doc(alias = "get_wl_output")]
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    pub fn wl_output(&self) -> Option<WlOutput> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        unsafe {
            let display_ptr = ffi::gdk_wayland_display_get_wl_display(display.to_glib_none().0);
            let output_ptr = ffi::gdk_wayland_monitor_get_wl_output(self.to_glib_none().0);
            if output_ptr.is_null() {
                None
            } else {
                let backend = wayland_backend::sys::client::Backend::from_foreign_display(
                    display_ptr as *mut _,
                );
                let cnx = wayland_client::Connection::from_backend(backend);
                let output_id =
                    ObjectId::from_ptr(&WlOutput::interface(), output_ptr as *mut _).unwrap();

                WlOutput::from_id(&cnx, output_id).ok()
            }
        }
    }
}
