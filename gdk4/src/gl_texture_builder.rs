// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{ffi, GLContext, GLTextureBuilder, MemoryFormat, Texture};

#[cfg(not(feature = "gl"))]
pub type GLsync = *const libc::c_void;

#[cfg(feature = "gl")]
pub use gl::types::GLsync;

impl GLTextureBuilder {
    #[doc(alias = "gdk_gl_texture_builder_build")]
    #[must_use = "The builder must be built to be used"]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn build(self) -> Texture {
        from_glib_full(ffi::gdk_gl_texture_builder_build(
            self.to_glib_none().0,
            None,
            std::ptr::null_mut(),
        ))
    }

    #[doc(alias = "gdk_gl_texture_builder_build")]
    #[must_use = "The builder must be built to be used"]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn build_with_release_func<F: FnOnce() + Send + 'static>(
        self,
        release_func: F,
    ) -> Texture {
        unsafe extern "C" fn destroy_closure<F: FnOnce() + Send + 'static>(
            func: glib::ffi::gpointer,
        ) {
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

    #[doc(alias = "gdk_gl_texture_builder_set_context")]
    pub fn set_context(self, context: Option<&impl IsA<GLContext>>) -> Self {
        unsafe {
            ffi::gdk_gl_texture_builder_set_context(
                self.to_glib_none().0,
                context.map(|p| p.as_ref()).to_glib_none().0,
            );
        }

        self
    }

    #[doc(alias = "gdk_gl_texture_builder_set_format")]
    pub fn set_format(self, format: MemoryFormat) -> Self {
        unsafe {
            ffi::gdk_gl_texture_builder_set_format(self.to_glib_none().0, format.into_glib());
        }

        self
    }

    #[doc(alias = "gdk_gl_texture_builder_set_has_mipmap")]
    pub fn set_has_mipmap(self, has_mipmap: bool) -> Self {
        unsafe {
            ffi::gdk_gl_texture_builder_set_has_mipmap(
                self.to_glib_none().0,
                has_mipmap.into_glib(),
            );
        }

        self
    }

    #[doc(alias = "gdk_gl_texture_builder_set_height")]
    pub fn set_height(self, height: i32) -> Self {
        unsafe {
            ffi::gdk_gl_texture_builder_set_height(self.to_glib_none().0, height);
        }

        self
    }

    #[doc(alias = "gdk_gl_texture_builder_set_id")]
    pub fn set_id(self, id: u32) -> Self {
        unsafe {
            ffi::gdk_gl_texture_builder_set_id(self.to_glib_none().0, id);
        }

        self
    }

    #[doc(alias = "gdk_gl_texture_builder_set_update_region")]
    pub fn set_update_region(self, region: Option<&cairo::Region>) -> Self {
        unsafe {
            ffi::gdk_gl_texture_builder_set_update_region(
                self.to_glib_none().0,
                mut_override(region.to_glib_none().0),
            );
        }

        self
    }

    #[doc(alias = "gdk_gl_texture_builder_set_update_texture")]
    pub fn set_update_texture(self, texture: Option<&impl IsA<Texture>>) -> Self {
        unsafe {
            ffi::gdk_gl_texture_builder_set_update_texture(
                self.to_glib_none().0,
                texture.map(|p| p.as_ref()).to_glib_none().0,
            );
        }

        self
    }

    #[doc(alias = "gdk_gl_texture_builder_set_width")]
    pub fn set_width(self, width: i32) -> Self {
        unsafe {
            ffi::gdk_gl_texture_builder_set_width(self.to_glib_none().0, width);
        }

        self
    }

    #[doc(alias = "gdk_gl_texture_builder_get_sync")]
    #[doc(alias = "get_sync")]
    pub fn sync(&self) -> Option<GLsync> {
        let ptr = unsafe { ffi::gdk_gl_texture_builder_get_sync(self.to_glib_none().0) };
        if ptr.is_null() {
            None
        } else {
            Some(ptr as _)
        }
    }

    #[doc(alias = "gdk_gl_texture_builder_set_sync")]
    pub fn set_sync(self, sync: Option<GLsync>) -> Self {
        let ptr = sync.unwrap_or(std::ptr::null());
        unsafe {
            ffi::gdk_gl_texture_builder_set_sync(self.to_glib_none().0, ptr as _);
        }

        self
    }
}
