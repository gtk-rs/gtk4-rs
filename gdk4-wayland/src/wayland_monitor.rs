// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WaylandMonitor;
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use glib::translate::ToGlibPtr;
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use wayland_client::{protocol::wl_output::WlOutput, sys::client::wl_proxy, Proxy};

impl WaylandMonitor {
    #[doc(alias = "gdk_wayland_monitor_get_wl_output")]
    #[doc(alias = "get_wl_output")]
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    pub fn wl_output(&self) -> WlOutput {
        unsafe {
            let ptr = ffi::gdk_wayland_monitor_get_wl_output(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }
}
