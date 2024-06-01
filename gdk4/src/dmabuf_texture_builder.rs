// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, DmabufTextureBuilder, Texture};

impl DmabufTextureBuilder {
    #[doc(alias = "gdk_dmabuf_texture_builder_build")]
    #[must_use = "The builder must be built to be used"]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn build(self) -> Result<Texture, glib::Error> {
        let mut error = std::ptr::null_mut();

        let result = ffi::gdk_dmabuf_texture_builder_build(
            self.to_glib_none().0,
            None,
            std::ptr::null_mut(),
            &mut error,
        );
        if error.is_null() {
            if result.is_null() {
                Err(glib::Error::new(
                    crate::DmabufError::UnsupportedFormat,
                    "Unsupported format",
                ))
            } else {
                Ok(from_glib_full(result))
            }
        } else {
            Err(from_glib_full(error))
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_build")]
    #[must_use = "The builder must be built to be used"]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn build_with_release_func<F: FnOnce() + Send + 'static>(
        self,
        release_func: F,
    ) -> Result<Texture, glib::Error> {
        let mut error = std::ptr::null_mut();
        unsafe extern "C" fn destroy_closure<F: FnOnce() + Send + 'static>(
            func: glib::ffi::gpointer,
        ) {
            let released_func = Box::<F>::from_raw(func as *mut _);
            released_func();
        }
        let released_func = Box::new(release_func);
        let result = ffi::gdk_dmabuf_texture_builder_build(
            self.to_glib_none().0,
            Some(destroy_closure::<F>),
            Box::into_raw(released_func) as glib::ffi::gpointer,
            &mut error,
        );
        if error.is_null() {
            if result.is_null() {
                Err(glib::Error::new(
                    crate::DmabufError::UnsupportedFormat,
                    "Unsupported format",
                ))
            } else {
                Ok(from_glib_full(result))
            }
        } else {
            Err(from_glib_full(error))
        }
    }
}
