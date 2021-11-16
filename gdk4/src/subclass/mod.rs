// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for creating custom types.

pub mod content_provider;
pub mod paintable;

// rustdoc-stripper-ignore-next
/// Traits intended for blanket imports.
pub mod prelude {
    #[doc(hidden)]
    pub use gio::subclass::prelude::*;
    #[doc(hidden)]
    pub use glib::subclass::prelude::*;

    pub use super::content_provider::{ContentProviderImpl, ContentProviderImplExt};
    pub use super::paintable::{PaintableImpl, PaintableImplExt};
}
