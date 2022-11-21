// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ContentSerializer;
use glib::translate::*;
use glib::value::FromValue;

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

    // rustdoc-stripper-ignore-next
    /// Similar to [`Self::value`] but panics if the value is of a different type.
    #[doc(alias = "gdk_content_serializer_get_value")]
    #[doc(alias = "get_value")]
    pub fn value_as<V: for<'b> FromValue<'b> + 'static>(&self) -> V {
        self.value()
            .get_owned::<V>()
            .expect("Failed to get the value")
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
