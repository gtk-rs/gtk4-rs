use crate::ffi;
use crate::WaylandDevice;
use glib::translate::ToGlibPtr;
use glib::IsA;
use wayland_client::protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer, wl_seat::WlSeat};
use wayland_client::sys::client::wl_proxy;
use wayland_client::Proxy;

pub trait WaylandDeviceManualExt {
    fn get_wl_keyboard(&self) -> WlKeyboard;
    fn get_wl_pointer(&self) -> WlPointer;
    fn get_wl_seat(&self) -> WlSeat;
}

impl<O: IsA<WaylandDevice>> WaylandDeviceManualExt for O {
    fn get_wl_keyboard(&self) -> WlKeyboard {
        unsafe {
            let ptr = ffi::gdk_wayland_device_get_wl_keyboard(self.as_ref().to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }

    fn get_wl_pointer(&self) -> WlPointer {
        unsafe {
            let ptr = ffi::gdk_wayland_device_get_wl_pointer(self.as_ref().to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }

    fn get_wl_seat(&self) -> WlSeat {
        unsafe {
            let ptr = ffi::gdk_wayland_device_get_wl_seat(self.as_ref().to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }
}
