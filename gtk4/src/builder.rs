// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Builder};
use glib::{translate::*, IntoGStr, Object};
use std::path::Path;

impl Builder {
    #[doc(alias = "gtk_builder_new_from_file")]
    #[doc(alias = "new_from_file")]
    pub fn from_file(file_path: impl AsRef<Path>) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_builder_new_from_file(
                file_path.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_builder_get_current_object")]
    #[doc(alias = "get_current_object")]
    pub fn current_object(&self) -> Option<Object> {
        unsafe {
            let ptr = ffi::gtk_builder_get_current_object(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                glib::gobject_ffi::g_object_ref(ptr as *mut glib::gobject_ffi::GObject);
                Some(from_glib_full(ptr))
            }
        }
    }

    #[doc(alias = "gtk_builder_get_object")]
    #[doc(alias = "get_object")]
    pub fn object<T: IsA<Object>>(&self, name: impl IntoGStr) -> Option<T> {
        unsafe {
            name.run_with_gstr(|name| {
                Option::<Object>::from_glib_none(ffi::gtk_builder_get_object(
                    self.to_glib_none().0,
                    name.as_ptr(),
                ))
                .and_then(|obj| obj.dynamic_cast::<T>().ok())
            })
        }
    }

    #[doc(alias = "gtk_builder_extend_with_template")]
    pub fn extend_with_template<T: IsA<Object>>(
        &self,
        object: &impl IsA<glib::Object>,
        buffer: &str,
    ) -> Result<(), glib::Error> {
        self.extend_with_template_and_type(object, T::static_type(), buffer)
    }

    #[doc(alias = "gtk_builder_extend_with_template")]
    pub fn extend_with_template_and_type(
        &self,
        object: &impl IsA<glib::Object>,
        template_type: glib::Type,
        buffer: &str,
    ) -> Result<(), glib::Error> {
        let length = buffer.len() as _;
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gtk_builder_extend_with_template(
                self.to_glib_none().0,
                object.as_ref().to_glib_none().0,
                template_type.into_glib(),
                buffer.to_glib_none().0,
                length,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_builder_value_from_string_type")]
    pub fn value_from_string_type<T: for<'a> glib::value::FromValue<'a> + StaticType>(
        &self,
        string: &str,
    ) -> Result<T, glib::Error> {
        self.value_from_string_type_with_type(T::static_type(), string)
            .map(|v| v.get::<T>().unwrap())
    }

    #[doc(alias = "gtk_builder_value_from_string_type")]
    pub fn value_from_string_type_with_type(
        &self,
        type_: glib::Type,
        string: &str,
    ) -> Result<glib::Value, glib::Error> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gtk_builder_value_from_string_type(
                self.to_glib_none().0,
                type_.into_glib(),
                string.to_glib_none().0,
                value.to_glib_none_mut().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(value)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_builder_add_from_file")]
    pub fn add_from_file(&self, file_path: impl AsRef<Path>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            ffi::gtk_builder_add_from_file(
                self.to_glib_none().0,
                file_path.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
