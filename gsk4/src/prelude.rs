// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for blanket imports.

pub use crate::auto::traits::*;

pub use crate::render_node::IsRenderNode;

#[doc(hidden)]
pub use gdk::prelude::*;
#[doc(hidden)]
pub use glib::prelude::*;
#[doc(hidden)]
pub use graphene::prelude::*;
#[doc(hidden)]
pub use pango::prelude::*;
