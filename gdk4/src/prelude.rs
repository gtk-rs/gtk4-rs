// Take a look at the license at the top of the repository in the LICENSE file.

//! Traits intended for blanket imports.

pub use crate::auto::traits::*;

pub use crate::cairo_interaction::{GdkCairoContextExt, GdkCairoSurfaceExt};
pub use crate::draw_context::DrawContextExtManual;
#[doc(hidden)]
pub use glib::prelude::*;
