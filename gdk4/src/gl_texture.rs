// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
use crate::builders::GLTextureBuilder;
use crate::{ffi, GLContext, GLTexture};

impl GLTexture {
    #[doc(alias = "gdk_gl_texture_new")]
    #[allow(clippy::missing_safety_doc)]
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
    #[allow(clippy::missing_safety_doc)]
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

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GLTexture`]
    /// objects.
    ///
    /// This method returns an instance of
    /// [`GLTextureBuilder`](crate::builders::GLTextureBuilder) which can be
    /// used to create [`GLTexture`] objects.
    pub fn builder() -> GLTextureBuilder {
        GLTextureBuilder::new()
    }
}
