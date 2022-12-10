// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, ContentProvider};
use glib::translate::*;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`ContentProvider`](crate::ContentProvider).
pub trait ContentProviderExtManual {
    #[doc(alias = "gdk_content_provider_get_value")]
    fn value(&self, type_: glib::Type) -> Result<glib::Value, glib::Error>;
}

impl<O: IsA<ContentProvider>> ContentProviderExtManual for O {
    fn value(&self, type_: glib::Type) -> Result<glib::Value, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let mut value = glib::Value::from_type(type_);
            let _ = ffi::gdk_content_provider_get_value(
                self.as_ref().to_glib_none().0,
                value.to_glib_none_mut().0,
                &mut error,
            );
            if error.is_null() {
                Ok(value)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
