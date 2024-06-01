// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]
#![allow(deprecated)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use gdk;
pub use gdk4_wayland_sys as ffi;
pub use gio;
pub use glib;
#[cfg(all(feature = "v4_4", feature = "egl"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "v4_4", feature = "egl"))))]
pub use khronos_egl;
#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
pub use wayland_client;

mod auto;

pub mod prelude;
pub use auto::*;

mod wayland_device;
mod wayland_display;
mod wayland_monitor;
mod wayland_seat;
mod wayland_surface;
mod wayland_toplevel;
