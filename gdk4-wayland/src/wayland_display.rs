// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(feature = "wayland_crate", all(feature = "v4_4", feature = "egl")))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "wayland_crate", all(feature = "v4_4", feature = "egl"))))
)]
use crate::ffi;
#[cfg(any(feature = "wayland_crate", all(feature = "v4_4", feature = "egl")))]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use glib::translate::*;
#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use glib::{Quark, prelude::*};

#[cfg(all(feature = "v4_4", feature = "egl"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "v4_4", feature = "egl"))))]
use khronos_egl as egl;
#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
use wayland_client::{
    Proxy,
    backend::ObjectId,
    protocol::{wl_compositor::WlCompositor, wl_display::WlDisplay},
};

use crate::WaylandDisplay;

impl WaylandDisplay {
    #[cfg(all(feature = "v4_4", feature = "egl"))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "v4_4", feature = "egl"))))]
    #[doc(alias = "gdk_wayland_display_get_egl_display")]
    #[doc(alias = "get_egl_display")]
    pub fn egl_display(&self) -> Option<egl::Display> {
        unsafe {
            let ptr = ffi::gdk_wayland_display_get_egl_display(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(egl::Display::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "gdk_wayland_display_get_wl_compositor")]
    #[doc(alias = "get_wl_compositor")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    pub fn wl_compositor(&self) -> Option<WlCompositor> {
        unsafe {
            let compositor_ptr = ffi::gdk_wayland_display_get_wl_compositor(self.to_glib_none().0);
            if compositor_ptr.is_null() {
                None
            } else {
                let cnx = self.connection();
                let id = ObjectId::from_ptr(WlCompositor::interface(), compositor_ptr as *mut _)
                    .unwrap();

                WlCompositor::from_id(&cnx, id).ok()
            }
        }
    }

    #[doc(alias = "gdk_wayland_display_get_wl_display")]
    #[doc(alias = "get_wl_display")]
    #[cfg(feature = "wayland_crate")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
    pub fn wl_display(&self) -> Option<WlDisplay> {
        unsafe {
            let display_ptr = ffi::gdk_wayland_display_get_wl_display(self.to_glib_none().0);
            if display_ptr.is_null() {
                None
            } else {
                let cnx = self.connection();
                let id = ObjectId::from_ptr(WlDisplay::interface(), display_ptr as *mut _).unwrap();

                WlDisplay::from_id(&cnx, id).ok()
            }
        }
    }

    #[cfg(feature = "wayland_crate")]
    pub(crate) fn connection(&self) -> wayland_client::Connection {
        use std::sync::OnceLock;
        static QUARK: OnceLock<Quark> = OnceLock::new();
        let quark =
            *QUARK.get_or_init(|| Quark::from_str("gtk-rs-wayland-display-connection-quark"));

        unsafe {
            match self.qdata::<Option<wayland_client::Connection>>(quark) {
                Some(conn) => conn.as_ref().clone().unwrap(),
                None => {
                    let display_ptr =
                        ffi::gdk_wayland_display_get_wl_display(self.to_glib_none().0);
                    let backend = wayland_backend::sys::client::Backend::from_foreign_display(
                        display_ptr as *mut _,
                    );
                    let conn = wayland_client::Connection::from_backend(backend);
                    self.set_qdata(quark, conn.clone());

                    conn
                }
            }
        }
    }
}
