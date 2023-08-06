// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
#[cfg(feature = "wayland_crate")]
#[cfg_attr(docsrs, doc(cfg(feature = "wayland_crate")))]
pub use wayland_client;

#[cfg(all(feature = "v4_4", feature = "egl"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "v4_4", feature = "egl"))))]
pub use khronos_egl;

#[cfg(all(feature = "v4_4", feature = "xkb_crate"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "v4_4", feature = "xkb_crate"))))]
pub use xkb;

mod auto;

pub mod prelude;
pub use auto::*;

mod wayland_device;
mod wayland_display;
mod wayland_monitor;
mod wayland_seat;
mod wayland_surface;
