// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
#[cfg(any(feature = "xlib", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "xlib")))]
pub use x11;

#[cfg(any(all(feature = "v4_4", feature = "egl"), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(all(feature = "v4_4", feature = "egl"))))]
pub use khronos_egl;
#[macro_use]
mod rt;

#[allow(clippy::upper_case_acronyms)]
mod auto;

pub mod builders;
pub mod prelude;

mod functions;
mod x11_display;
mod x11_monitor;
mod x11_screen;
mod x11_surface;

pub use auto::functions::*;
pub use auto::*;
pub use functions::*;

#[cfg(not(feature = "xlib"))]
pub type XID = libc::c_ulong;
#[cfg(not(feature = "xlib"))]
pub type XWindow = XID;
#[cfg(not(feature = "xlib"))]
pub type XCursor = XID;
#[cfg(not(feature = "xlib"))]
pub type XAtom = XID;
