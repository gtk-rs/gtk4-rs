// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for blanket imports.

#[doc(hidden)]
pub use gdk::prelude::*;

pub use crate::auto::traits::*;
pub use crate::wayland_surface::WaylandSurfaceExtManual;
