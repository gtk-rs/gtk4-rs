// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! # Rust GDK 4 Wayland bindings
//!
//! This library contains safe Rust bindings for [GDK 4 Wayland](https://docs.gtk.org/gdk4-wayland/).
//!
//! GDK is an intermediate layer that isolates GTK from the details of the windowing system.
//! GDK Wayland contains functions specific to the Wayland backend.

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use wayland_client;

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
