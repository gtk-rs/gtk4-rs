// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for blanket imports.

#[doc(hidden)]
pub use gdk_pixbuf::prelude::*;
#[doc(hidden)]
pub use pango::prelude::*;

#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
pub use crate::drag_surface::DragSurfaceExtManual;
pub use crate::{
    auto::traits::*,
    cairo_interaction::{GdkCairoContextExt, GdkCairoSurfaceExt},
    content_provider::ContentProviderExtManual,
    display::DisplayExtManual,
    draw_context::DrawContextExtManual,
    event::EventKind,
    surface::SurfaceExtManual,
    texture::TextureExtManual,
    toplevel::ToplevelExtManual,
};
