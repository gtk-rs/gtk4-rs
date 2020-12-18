// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(deprecated)]

pub use ffi;
pub use x11;
#[macro_use]
mod rt;

#[allow(unused_imports)]
#[allow(clippy::let_and_return)]
mod auto;

pub use auto::*;
