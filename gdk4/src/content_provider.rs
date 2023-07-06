// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, ContentProvider};
use glib::translate::*;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ContentProvider>> Sealed for T {}
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`ContentProvider`](crate::ContentProvider).
pub trait ContentProviderExtManual: sealed::Sealed + IsA<ContentProvider> {
    fn value<T: for<'a> glib::value::FromValue<'a> + glib::StaticType>(
        &self,
    ) -> Result<T, glib::Error> {
        self.value_with_type(T::static_type())
            .map(|v| v.get::<T>().unwrap())
    }

    #[doc(alias = "gdk_content_provider_get_value")]
    fn value_with_type(&self, type_: glib::Type) -> Result<glib::Value, glib::Error> {
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

impl<O: IsA<ContentProvider>> ContentProviderExtManual for O {}
