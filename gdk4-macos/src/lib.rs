// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]
#![allow(deprecated)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use gdk;
pub use gdk4_macos_sys as ffi;
pub use gio;
pub use glib;
#[macro_use]
mod rt;

mod auto;

pub mod prelude;

pub use auto::*;
