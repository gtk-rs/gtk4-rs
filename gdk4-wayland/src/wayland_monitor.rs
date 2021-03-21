// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WaylandMonitor;
use glib::translate::ToGlibPtr;
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::sys::client::wl_proxy;
use wayland_client::Proxy;

impl WaylandMonitor {
    #[doc(alias = "gdk_wayland_monitor_get_wl_output")]
    pub fn get_wl_output(&self) -> WlOutput {
        unsafe {
            let ptr = ffi::gdk_wayland_monitor_get_wl_output(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }
}
