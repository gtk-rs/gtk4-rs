// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WaylandDevice;
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use glib::translate::ToGlibPtr;
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use wayland_client::{
    protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer, wl_seat::WlSeat},
    sys::client::wl_proxy,
    Proxy,
};

#[cfg(any(all(feature = "v4_4", feature = "xkb_crate"), feature = "dox"))]
#[cfg_attr(
    feature = "dox",
    doc(cfg(all(feature = "v4_4", feature = "xkb_crate")))
)]
use xkb::Keymap;

impl WaylandDevice {
    #[doc(alias = "gdk_wayland_device_get_wl_keyboard")]
    #[doc(alias = "get_wl_keyboard")]
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    pub fn wl_keyboard(&self) -> WlKeyboard {
        unsafe {
            let ptr = ffi::gdk_wayland_device_get_wl_keyboard(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_pointer")]
    #[doc(alias = "get_wl_pointer")]
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    pub fn wl_pointer(&self) -> WlPointer {
        unsafe {
            let ptr = ffi::gdk_wayland_device_get_wl_pointer(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_seat")]
    #[doc(alias = "get_wl_seat")]
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    pub fn wl_seat(&self) -> WlSeat {
        unsafe {
            let ptr = ffi::gdk_wayland_device_get_wl_seat(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }

    #[cfg(any(all(feature = "v4_4", feature = "xkb_crate"), feature = "dox"))]
    #[cfg_attr(
        feature = "dox",
        doc(cfg(all(feature = "v4_4", feature = "xkb_crate")))
    )]
    #[doc(alias = "gdk_wayland_device_get_xkb_keymap")]
    #[doc(alias = "get_xkb_keymap")]
    pub fn xkb_keymap(&self) -> Option<Keymap> {
        unsafe {
            let ptr = ffi::gdk_wayland_device_get_xkb_keymap(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(Keymap::from_ptr(ptr))
            }
        }
    }
}
