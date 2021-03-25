// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(deprecated)]

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use x11;

#[macro_use]
mod rt;

#[allow(unused_imports)]
#[allow(clippy::let_and_return)]
#[allow(clippy::upper_case_acronyms)]
mod auto;

pub use auto::*;
