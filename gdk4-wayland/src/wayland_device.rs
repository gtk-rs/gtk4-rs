// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::c_void;
use std::ptr::NonNull;

use glib::translate::*;
#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use wayland_client::{
    Proxy,
    backend::ObjectId,
    protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer, wl_seat::WlSeat},
};

#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use crate::prelude::*;
use crate::{WaylandDevice, ffi};

impl WaylandDevice {
    #[doc(alias = "gdk_wayland_device_get_wl_keyboard")]
    #[doc(alias = "get_wl_keyboard")]
    pub fn wl_keyboard_raw(&self) -> Option<NonNull<c_void>> {
        NonNull::new(unsafe { ffi::gdk_wayland_device_get_wl_keyboard(self.to_glib_none().0) })
    }

    #[doc(alias = "gdk_wayland_device_get_wl_keyboard")]
    #[doc(alias = "get_wl_keyboard")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    pub fn wl_keyboard(&self) -> Option<WlKeyboard> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        let ptr = self.wl_keyboard_raw()?;
        unsafe {
            let cnx = display.connection();
            let id = ObjectId::from_ptr(WlKeyboard::interface(), ptr.as_ptr() as *mut _).unwrap();

            WlKeyboard::from_id(&cnx, id).ok()
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_pointer")]
    #[doc(alias = "get_wl_pointer")]
    pub fn wl_pointer_raw(&self) -> Option<NonNull<c_void>> {
        NonNull::new(unsafe { ffi::gdk_wayland_device_get_wl_pointer(self.to_glib_none().0) })
    }

    #[doc(alias = "gdk_wayland_device_get_wl_pointer")]
    #[doc(alias = "get_wl_pointer")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    pub fn wl_pointer(&self) -> Option<WlPointer> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        let ptr = self.wl_pointer_raw()?;
        unsafe {
            let cnx = display.connection();
            let id = ObjectId::from_ptr(WlPointer::interface(), ptr.as_ptr() as *mut _).unwrap();

            WlPointer::from_id(&cnx, id).ok()
        }
    }

    #[doc(alias = "gdk_wayland_device_get_wl_seat")]
    #[doc(alias = "get_wl_seat")]
    pub fn wl_seat_raw(&self) -> Option<NonNull<c_void>> {
        NonNull::new(unsafe { ffi::gdk_wayland_device_get_wl_seat(self.to_glib_none().0) })
    }

    #[doc(alias = "gdk_wayland_device_get_wl_seat")]
    #[doc(alias = "get_wl_seat")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    pub fn wl_seat(&self) -> Option<WlSeat> {
        let display = self.display().downcast::<crate::WaylandDisplay>().unwrap();
        let ptr = self.wl_seat_raw()?;
        unsafe {
            let cnx = display.connection();
            let id = ObjectId::from_ptr(WlSeat::interface(), ptr.as_ptr() as *mut _).unwrap();

            WlSeat::from_id(&cnx, id).ok()
        }
    }
}
