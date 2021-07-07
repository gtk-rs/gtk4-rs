// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use wayland_client;

#[cfg(any(feature = "v4_4", feature = "dox"))]
pub use khronos_egl;

#[cfg(any(feature = "v4_4", feature = "dox"))]
pub use xkb;

mod auto;

pub mod prelude;
pub use auto::*;

mod wayland_device;
mod wayland_display;
mod wayland_monitor;
mod wayland_seat;
mod wayland_surface;
