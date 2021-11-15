// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use x11;

#[macro_use]
mod rt;

#[allow(clippy::upper_case_acronyms)]
#[allow(unused_imports)]
mod auto;

pub mod builders;

mod x11_display;

pub use auto::*;
