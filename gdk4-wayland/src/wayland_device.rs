// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(feature = "wayland_crate", feature = "xkb_crate"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "wayland_crate", feature = "xkb_crate")))
)]
use glib::translate::*;
#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use wayland_client::{
    backend::ObjectId,
    protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer, wl_seat::WlSeat},
    Proxy,
};
#[cfg(all(feature = "v4_4", feature = "xkb_crate"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "v4_4", feature = "xkb_crate"))))]
use xkb::Keymap;

#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use crate::prelude::*;
use crate::WaylandDevice;

impl WaylandDevice {
    #[doc(alias = "gdk_wayland_device_get_wl_keyboard")]
    #[doc(alias = "get_wl_keyboard")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    pub fn wl_keyboard(&self) -> Option<WlKeyboard> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        unsafe {
            let keyboard_ptr = ffi::gdk_wayland_device_get_wl_keyboard(self.to_glib_none().0);
            if keyboard_ptr.is_null() {
                None
            } else {
                let cnx = display.connection();
                let id =
                    ObjectId::from_ptr(WlKeyboard::interface(), keyboard_ptr as *mut _).unwrap();

                WlKeyboard::from_id(&cnx, id).ok()
            }
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_pointer")]
    #[doc(alias = "get_wl_pointer")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    pub fn wl_pointer(&self) -> Option<WlPointer> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        unsafe {
            let pointer_ptr = ffi::gdk_wayland_device_get_wl_pointer(self.to_glib_none().0);
            if pointer_ptr.is_null() {
                None
            } else {
                let cnx = display.connection();
                let id = ObjectId::from_ptr(WlPointer::interface(), pointer_ptr as *mut _).unwrap();

                WlPointer::from_id(&cnx, id).ok()
            }
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_seat")]
    #[doc(alias = "get_wl_seat")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    pub fn wl_seat(&self) -> Option<WlSeat> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        unsafe {
            let seat_ptr = ffi::gdk_wayland_device_get_wl_seat(self.to_glib_none().0);
            if seat_ptr.is_null() {
                None
            } else {
                let cnx = display.connection();
                let id = ObjectId::from_ptr(WlSeat::interface(), seat_ptr as *mut _).unwrap();

                WlSeat::from_id(&cnx, id).ok()
            }
        }
    }

    #[cfg(all(feature = "v4_4", feature = "xkb_crate"))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "v4_4", feature = "xkb_crate"))))]
    #[doc(alias = "gdk_wayland_device_get_xkb_keymap")]
    #[doc(alias = "get_xkb_keymap")]
    pub fn xkb_keymap(&self) -> Option<Keymap> {
        unsafe {
            let ptr = ffi::gdk_wayland_device_get_xkb_keymap(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(Keymap::from_ptr(ptr as _))
            }
        }
    }
}
