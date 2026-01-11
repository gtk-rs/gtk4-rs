// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]
#![allow(deprecated)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(not(feature = "cocoa"))]
use std::ffi::c_void;

pub use gdk;
pub use gdk4_macos_sys as ffi;
pub use gio;
pub use glib;
#[macro_use]
mod rt;

#[allow(clippy::let_and_return)]
mod auto;

pub mod prelude;

pub use auto::*;

mod macos_surface;

#[cfg(not(feature = "cocoa"))]
#[allow(non_camel_case_types)]
pub type id = *mut c_void;

#[cfg(feature = "cocoa")]
pub use cocoa::base::id;
