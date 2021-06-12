// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! # Rust GDK 4 X11 bindings
//!
//! This library contains safe Rust bindings for [GDK 4 X11](https://docs.gtk.org/gdk4-x11/).
//!
//! GDK is an intermediate layer that isolates GTK from the details of the windowing system.
//! GDK X11 contains functions specific to the X11 backend.

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use x11;

#[macro_use]
mod rt;

#[allow(clippy::upper_case_acronyms)]
mod auto;

mod x11_display;

pub use auto::*;
