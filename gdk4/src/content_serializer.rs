// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ContentSerializer;
use glib::translate::*;

impl ContentSerializer {
    #[doc(alias = "gdk_content_serializer_get_priority")]
    #[doc(alias = "get_priority")]
    pub fn priority(&self) -> glib::Priority {
        unsafe {
            from_glib(ffi::gdk_content_serializer_get_priority(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_serializer_return_error")]
    pub fn return_error(&self, error: glib::Error) {
        unsafe {
            ffi::gdk_content_serializer_return_error(
                self.to_glib_none().0,
                mut_override(error.to_glib_full()),
            );
        }
    }
}
