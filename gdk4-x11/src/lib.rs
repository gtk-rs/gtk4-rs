// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
#[cfg(feature = "xlib")]
#[cfg_attr(docsrs, doc(cfg(feature = "xlib")))]
pub use x11;

#[cfg(all(feature = "v4_4", feature = "egl"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "v4_4", feature = "egl"))))]
pub use khronos_egl;
#[macro_use]
mod rt;

#[allow(clippy::upper_case_acronyms)]
#[allow(unused_imports)]
mod auto;

pub mod builders;
pub mod prelude;

mod functions;
mod x11_display;
mod x11_monitor;
mod x11_screen;
mod x11_surface;

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
