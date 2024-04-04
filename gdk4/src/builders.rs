// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Builder pattern types.

#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
pub use crate::GLTextureBuilder;
pub use crate::{auto::builders::*, rgba::RGBABuilder, ContentFormatsBuilder};
