// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ContentFormats, ContentFormatsBuilder};
use glib::{translate::*, Slice};

impl ContentFormats {
    #[doc(alias = "gdk_content_formats_get_gtypes")]
    #[doc(alias = "get_gtypes")]
    pub fn types(&self) -> Slice<glib::Type> {
        unsafe {
            let mut n_types = std::mem::MaybeUninit::uninit();
            let types =
                ffi::gdk_content_formats_get_gtypes(self.to_glib_none().0, n_types.as_mut_ptr());

            Slice::from_glib_none_num(types, n_types.assume_init() as _)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ContentFormats`] objects.
    ///
    /// This method returns an instance of [`ContentFormatsBuilder`](crate::builders::ContentFormatsBuilder) which can be used to create [`ContentFormats`] objects.
    pub fn builder() -> ContentFormatsBuilder {
        ContentFormatsBuilder::default()
    }
}

#[cfg(any(feature = "v4_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
impl std::str::FromStr for ContentFormats {
    type Err = glib::BoolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();
        ContentFormats::parse(s)
    }
}
