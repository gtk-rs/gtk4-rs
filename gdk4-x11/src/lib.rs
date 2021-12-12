// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use x11;

#[cfg(any(all(feature = "v4_4", feature = "egl"), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(all(feature = "v4_4", feature = "egl"))))]
pub use khronos_egl;
#[macro_use]
mod rt;

#[allow(clippy::upper_case_acronyms)]
#[allow(unused_imports)]
mod auto;

pub mod builders;

mod x11_display;
mod x11_screen;

pub use auto::*;
