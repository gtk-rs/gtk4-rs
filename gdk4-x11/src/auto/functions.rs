// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, X11DeviceManagerXI2, X11DeviceXI2, X11Surface};
use glib::translate::*;

#[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
#[allow(deprecated)]
#[doc(alias = "gdk_x11_device_get_id")]
pub fn x11_device_get_id(device: &X11DeviceXI2) -> i32 {
    skip_assert_initialized!();
    unsafe { ffi::gdk_x11_device_get_id(device.to_glib_none().0) }
}

#[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
#[allow(deprecated)]
#[doc(alias = "gdk_x11_device_manager_lookup")]
pub fn x11_device_manager_lookup(
    device_manager: &X11DeviceManagerXI2,
    device_id: i32,
) -> Option<X11DeviceXI2> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gdk_x11_device_manager_lookup(
            device_manager.to_glib_none().0,
            device_id,
        ))
    }
}

#[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
#[allow(deprecated)]
#[doc(alias = "gdk_x11_get_server_time")]
pub fn x11_get_server_time(surface: &X11Surface) -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gdk_x11_get_server_time(surface.to_glib_none().0) }
}

#[doc(alias = "gdk_x11_set_sm_client_id")]
pub fn x11_set_sm_client_id(sm_client_id: Option<&str>) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_x11_set_sm_client_id(sm_client_id.to_glib_none().0);
    }
}
