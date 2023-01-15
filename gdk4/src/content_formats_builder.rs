// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ContentFormats, ContentFormatsBuilder};
use glib::{translate::*, IntoGStr};

impl ContentFormatsBuilder {
    #[doc(alias = "gdk_content_formats_builder_add_formats")]
    #[must_use]
    pub fn add_formats(self, formats: &ContentFormats) -> Self {
        unsafe {
            ffi::gdk_content_formats_builder_add_formats(
                self.to_glib_none().0,
                formats.to_glib_none().0,
            );
        }

        self
    }

    #[doc(alias = "gdk_content_formats_builder_add_gtype")]
    #[must_use]
    pub fn add_type(self, type_: glib::types::Type) -> Self {
        unsafe {
            ffi::gdk_content_formats_builder_add_gtype(self.to_glib_none().0, type_.into_glib());
        }

        self
    }

    #[doc(alias = "gdk_content_formats_builder_add_mime_type")]
    #[must_use]
    pub fn add_mime_type(self, mime_type: impl IntoGStr) -> Self {
        unsafe {
            mime_type.run_with_gstr(|mime_type| {
                ffi::gdk_content_formats_builder_add_mime_type(
                    self.to_glib_none().0,
                    mime_type.as_ptr(),
                );
            });
        }

        self
    }

    #[doc(alias = "gdk_content_formats_builder_to_formats")]
    #[must_use = "The builder must be built to be used"]
    pub fn build(self) -> ContentFormats {
        unsafe {
            from_glib_full(ffi::gdk_content_formats_builder_to_formats(
                self.to_glib_none().0,
            ))
        }
    }
}
