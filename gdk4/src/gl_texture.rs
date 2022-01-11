// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{GLContext, GLTexture};
use glib::translate::*;

impl GLTexture {
    #[doc(alias = "gdk_gl_texture_new")]
    pub unsafe fn new(context: &GLContext, id: u32, width: i32, height: i32) -> Self {
        from_glib_full(ffi::gdk_gl_texture_new(
            context.to_glib_none().0,
            id,
            width,
            height,
            None,
            std::ptr::null_mut(),
        ))
    }

    #[doc(alias = "gdk_gl_texture_new")]
    pub unsafe fn with_release_func<F: FnOnce() + 'static>(
        context: &GLContext,
        id: u32,
        width: i32,
        height: i32,
        release_func: F,
    ) -> Self {
        unsafe extern "C" fn destroy_closure<F: FnOnce() + 'static>(func: glib::ffi::gpointer) {
            let released_func = Box::<F>::from_raw(func as *mut _);
            released_func();
        }
        let released_func = Box::new(release_func);
        from_glib_full(ffi::gdk_gl_texture_new(
            context.to_glib_none().0,
            id,
            width,
            height,
            Some(destroy_closure::<F>),
            Box::into_raw(released_func) as glib::ffi::gpointer,
        ))
    }
}
