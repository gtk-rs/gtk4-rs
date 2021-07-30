// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]

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
