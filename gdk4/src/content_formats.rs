// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ContentFormats, ContentFormatsBuilder};
use glib::{translate::*, Slice, StaticType};

impl ContentFormats {
    #[doc(alias = "gdk_content_formats_new_for_gtype")]
    #[doc(alias = "new_for_gtype")]
    pub fn for_type<T: StaticType>() -> Self {
        assert_initialized_main_thread!();
        Self::for_type_with_type(T::static_type())
    }

    #[doc(alias = "gdk_content_formats_new_for_gtype")]
    #[doc(alias = "new_for_gtype")]
    pub fn for_type_with_type(type_: glib::Type) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gdk_content_formats_new_for_gtype(type_.into_glib())) }
    }

    #[doc(alias = "gdk_content_formats_contain_gtype")]
    #[doc(alias = "contain_gtype")]
    pub fn contains_type<T: StaticType>(&self) -> bool {
        self.contains_type_with_type(T::static_type())
    }

    #[doc(alias = "gdk_content_formats_contain_gtype")]
    #[doc(alias = "contain_gtype")]
    pub fn contains_type_with_type(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(ffi::gdk_content_formats_contain_gtype(
                self.to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

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

#[cfg(any(feature = "v4_4", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
impl std::str::FromStr for ContentFormats {
    type Err = glib::BoolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();
        ContentFormats::parse(s)
    }
}
