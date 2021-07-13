// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ContentFormats;
use glib::translate::*;

impl ContentFormats {
    #[doc(alias = "gdk_content_formats_get_gtypes")]
    #[doc(alias = "get_gtypes")]
    pub fn types(&self) -> Vec<glib::Type> {
        unsafe {
            let mut n_types = std::mem::MaybeUninit::uninit();
            let types =
                ffi::gdk_content_formats_get_gtypes(self.to_glib_none().0, n_types.as_mut_ptr());

            FromGlibContainer::from_glib_none_num(types, n_types.assume_init() as usize)
        }
    }

    #[doc(alias = "gdk_content_formats_get_mime_types")]
    pub fn mime_types(&self) -> Vec<glib::GString> {
        unsafe {
            let mut n_mime_types = std::mem::MaybeUninit::uninit();
            let mime_types = ffi::gdk_content_formats_get_mime_types(
                self.to_glib_none().0,
                n_mime_types.as_mut_ptr(),
            );
            FromGlibContainer::from_glib_none_num(mime_types, n_mime_types.assume_init() as usize)
        }
    }
}
