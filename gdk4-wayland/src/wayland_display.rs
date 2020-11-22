use crate::WaylandDisplay;
use glib::translate::ToGlibPtr;
use glib::IsA;
use wayland_client::protocol::{wl_compositor::WlCompositor, wl_display::WlDisplay};
use wayland_client::sys::client::wl_proxy;
use wayland_client::Proxy;

pub trait WaylandDisplayManualExt {
    fn get_wl_compositor(&self) -> WlCompositor;
    fn get_wl_display(&self) -> WlDisplay;
}

impl<O: IsA<WaylandDisplay>> WaylandDisplayManualExt for O {
    fn get_wl_compositor(&self) -> WlCompositor {
        unsafe {
            let ptr = ffi::gdk_wayland_display_get_wl_compositor(self.as_ref().to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }

    fn get_wl_display(&self) -> WlDisplay {
        unsafe {
            let ptr = ffi::gdk_wayland_display_get_wl_display(self.as_ref().to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }
}
