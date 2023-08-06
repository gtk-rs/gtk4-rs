// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for blanket imports.

pub use crate::auto::traits::*;

pub use crate::cairo_interaction::{GdkCairoContextExt, GdkCairoSurfaceExt};
pub use crate::content_provider::ContentProviderExtManual;
pub use crate::display::DisplayExtManual;
#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
pub use crate::drag_surface::DragSurfaceExtManual;
pub use crate::draw_context::DrawContextExtManual;
pub use crate::event::EventKind;
pub use crate::surface::SurfaceExtManual;
pub use crate::texture::TextureExtManual;
pub use crate::toplevel::ToplevelExtManual;

#[doc(hidden)]
pub use gdk_pixbuf::prelude::*;
#[doc(hidden)]
pub use gio::prelude::*;
#[doc(hidden)]
pub use glib::prelude::*;
#[doc(hidden)]
pub use pango::prelude::*;
