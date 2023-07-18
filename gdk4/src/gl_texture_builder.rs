// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{GLTextureBuilder, Texture};
use glib::translate::*;

impl GLTextureBuilder {
    #[doc(alias = "gdk_gl_texture_builder_build")]
    pub unsafe fn build(&self) -> Texture {
        from_glib_full(ffi::gdk_gl_texture_builder_build(
            self.to_glib_none().0,
            None,
            std::ptr::null_mut(),
        ))
    }

    #[doc(alias = "gdk_gl_texture_builder_build")]
    pub unsafe fn build_with_release_func<F: FnOnce() + 'static>(
        &self,
        release_func: F,
    ) -> Texture {
        unsafe extern "C" fn destroy_closure<F: FnOnce() + 'static>(func: glib::ffi::gpointer) {
            let released_func = Box::<F>::from_raw(func as *mut _);
            released_func();
        }
        let released_func = Box::new(release_func);
        from_glib_full(ffi::gdk_gl_texture_builder_build(
            self.to_glib_none().0,
            Some(destroy_closure::<F>),
            Box::into_raw(released_func) as glib::ffi::gpointer,
        ))
    }
}
