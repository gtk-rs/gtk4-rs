// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WaylandDevice;
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use crate::{gdk::prelude::*, glib::translate::ToGlibPtr};
#[cfg(any(feature = "wayland_crate", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
use wayland_client::{
    backend::ObjectId,
    protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer, wl_seat::WlSeat},
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
    pub fn wl_keyboard(&self) -> Option<WlKeyboard> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        unsafe {
            let display_ptr = ffi::gdk_wayland_display_get_wl_display(display.to_glib_none().0);
            let keyboard_ptr = ffi::gdk_wayland_device_get_wl_keyboard(self.to_glib_none().0);
            if keyboard_ptr.is_null() {
                None
            } else {
                let backend = wayland_backend::sys::client::Backend::from_foreign_display(
                    display_ptr as *mut _,
                );
                let cnx = wayland_client::Connection::from_backend(backend);
                let keyboard_id =
                    ObjectId::from_ptr(&WlKeyboard::interface(), keyboard_ptr as *mut _).unwrap();

                WlKeyboard::from_id(&cnx, keyboard_id).ok()
            }
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_pointer")]
    #[doc(alias = "get_wl_pointer")]
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    pub fn wl_pointer(&self) -> Option<WlPointer> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        unsafe {
            let display_ptr = ffi::gdk_wayland_display_get_wl_display(display.to_glib_none().0);
            let pointer_ptr = ffi::gdk_wayland_device_get_wl_pointer(self.to_glib_none().0);
            if pointer_ptr.is_null() {
                None
            } else {
                let backend = wayland_backend::sys::client::Backend::from_foreign_display(
                    display_ptr as *mut _,
                );
                let cnx = wayland_client::Connection::from_backend(backend);
                let pointer_id =
                    ObjectId::from_ptr(&WlPointer::interface(), pointer_ptr as *mut _).unwrap();

                WlPointer::from_id(&cnx, pointer_id).ok()
            }
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_seat")]
    #[doc(alias = "get_wl_seat")]
    #[cfg(any(feature = "wayland_crate", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "wayland_crate")))]
    pub fn wl_seat(&self) -> Option<WlSeat> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        unsafe {
            let display_ptr = ffi::gdk_wayland_display_get_wl_display(display.to_glib_none().0);
            let seat_ptr = ffi::gdk_wayland_device_get_wl_seat(self.to_glib_none().0);
            if seat_ptr.is_null() {
                None
            } else {
                let backend = wayland_backend::sys::client::Backend::from_foreign_display(
                    display_ptr as *mut _,
                );
                let cnx = wayland_client::Connection::from_backend(backend);
                let seat_id = ObjectId::from_ptr(&WlSeat::interface(), seat_ptr as *mut _).unwrap();

                WlSeat::from_id(&cnx, seat_id).ok()
            }
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
