// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

#[cfg(feature = "v4_16")]
use crate::ColorState;
use crate::{ffi, Display, DmabufTextureBuilder, Texture};

impl DmabufTextureBuilder {
    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "gdk_dmabuf_texture_builder_set_color_state")]
    #[doc(alias = "color-state")]
    pub fn set_color_state(self, color_state: Option<&ColorState>) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_color_state(
                self.to_glib_none().0,
                color_state.to_glib_none().0,
            );
        }

        self
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_display")]
    #[doc(alias = "display")]
    pub fn set_display(self, display: &impl IsA<Display>) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_display(
                self.to_glib_none().0,
                display.as_ref().to_glib_none().0,
            );
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// # Safety
    ///
    /// The caller must ensure that `fd` says valid for at least as long as the texture, e.g. by
    /// using `build_with_release_func()` to get notified when `fd` is not used anymore.
    #[doc(alias = "gdk_dmabuf_texture_builder_set_fd")]
    pub unsafe fn set_fd(self, plane: u32, fd: std::os::fd::RawFd) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_fd(self.to_glib_none().0, plane, fd);
        }

        self
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_fd")]
    #[doc(alias = "get_fd")]
    pub fn fd(&self, plane: u32) -> Option<std::os::fd::BorrowedFd<'_>> {
        unsafe {
            let fd = ffi::gdk_dmabuf_texture_builder_get_fd(self.to_glib_none().0, plane);

            if fd == -1 {
                None
            } else {
                Some(std::os::fd::BorrowedFd::borrow_raw(fd))
            }
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_fourcc")]
    #[doc(alias = "fourcc")]
    pub fn set_fourcc(self, fourcc: u32) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_fourcc(self.to_glib_none().0, fourcc);
        }

        self
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_height")]
    #[doc(alias = "height")]
    pub fn set_height(self, height: u32) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_height(self.to_glib_none().0, height);
        }

        self
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_modifier")]
    #[doc(alias = "modifier")]
    pub fn set_modifier(self, modifier: u64) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_modifier(self.to_glib_none().0, modifier);
        }

        self
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_n_planes")]
    #[doc(alias = "n-planes")]
    pub fn set_n_planes(self, n_planes: u32) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_n_planes(self.to_glib_none().0, n_planes);
        }

        self
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_offset")]
    pub fn set_offset(self, plane: u32, offset: u32) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_offset(self.to_glib_none().0, plane, offset);
        }

        self
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_premultiplied")]
    #[doc(alias = "premultiplied")]
    pub fn set_premultiplied(self, premultiplied: bool) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_premultiplied(
                self.to_glib_none().0,
                premultiplied.into_glib(),
            );
        }

        self
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_stride")]
    pub fn set_stride(self, plane: u32, stride: u32) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_stride(self.to_glib_none().0, plane, stride);
        }

        self
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_update_region")]
    #[doc(alias = "update-region")]
    pub fn set_update_region(self, region: Option<&cairo::Region>) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_update_region(
                self.to_glib_none().0,
                mut_override(region.to_glib_none().0),
            );
        }

        self
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_update_texture")]
    #[doc(alias = "update-texture")]
    pub fn set_update_texture(self, texture: Option<&impl IsA<Texture>>) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_update_texture(
                self.to_glib_none().0,
                texture.map(|p| p.as_ref()).to_glib_none().0,
            );
        }

        self
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_width")]
    #[doc(alias = "width")]
    pub fn set_width(self, width: u32) -> Self {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_width(self.to_glib_none().0, width);
        }

        self
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_build")]
    #[must_use = "The builder must be built to be used"]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn build(self) -> Result<Texture, glib::Error> { unsafe {
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
    }}

    #[doc(alias = "gdk_dmabuf_texture_builder_build")]
    #[must_use = "The builder must be built to be used"]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn build_with_release_func<F: FnOnce() + Send + 'static>(
        self,
        release_func: F,
    ) -> Result<Texture, glib::Error> { unsafe {
        let mut error = std::ptr::null_mut();
        unsafe extern "C" fn destroy_closure<F: FnOnce() + Send + 'static>(
            func: glib::ffi::gpointer,
        ) { unsafe {
            let released_func = Box::<F>::from_raw(func as *mut _);
            released_func();
        }}
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
    }}
}
