// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, ColorState, MemoryFormat, MemoryTextureBuilder, Texture};
use glib::{prelude::*, translate::*};

impl MemoryTextureBuilder {
    #[doc(alias = "gdk_memory_texture_builder_set_bytes")]
    #[doc(alias = "bytes")]
    pub fn set_bytes(self, bytes: Option<&glib::Bytes>) -> Self {
        unsafe {
            ffi::gdk_memory_texture_builder_set_bytes(
                self.to_glib_none().0,
                bytes.to_glib_none().0,
            );
        }

        self
    }

    #[doc(alias = "gdk_memory_texture_builder_set_color_state")]
    #[doc(alias = "color-state")]
    pub fn set_color_state(self, color_state: Option<&ColorState>) -> Self {
        unsafe {
            ffi::gdk_memory_texture_builder_set_color_state(
                self.to_glib_none().0,
                color_state.to_glib_none().0,
            );
        }

        self
    }

    #[doc(alias = "gdk_memory_texture_builder_set_format")]
    #[doc(alias = "format")]
    pub fn set_format(self, format: MemoryFormat) -> Self {
        unsafe {
            ffi::gdk_memory_texture_builder_set_format(self.to_glib_none().0, format.into_glib());
        }

        self
    }

    #[doc(alias = "gdk_memory_texture_builder_set_height")]
    #[doc(alias = "height")]
    pub fn set_height(self, height: i32) -> Self {
        unsafe {
            ffi::gdk_memory_texture_builder_set_height(self.to_glib_none().0, height);
        }

        self
    }

    #[cfg(feature = "v4_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_20")))]
    #[doc(alias = "gdk_memory_texture_builder_set_offset")]
    pub fn set_offset(self, plane: u32, offset: usize) -> Self {
        unsafe {
            ffi::gdk_memory_texture_builder_set_offset(self.to_glib_none().0, plane, offset);
        }

        self
    }

    #[doc(alias = "gdk_memory_texture_builder_set_stride")]
    #[doc(alias = "stride")]
    pub fn set_stride(self, stride: usize) -> Self {
        unsafe {
            ffi::gdk_memory_texture_builder_set_stride(self.to_glib_none().0, stride);
        }

        self
    }

    #[cfg(feature = "v4_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_20")))]
    #[doc(alias = "gdk_memory_texture_builder_set_stride_for_plane")]
    pub fn set_stride_for_plane(self, plane: u32, stride: usize) -> Self {
        unsafe {
            ffi::gdk_memory_texture_builder_set_stride_for_plane(
                self.to_glib_none().0,
                plane,
                stride,
            );
        }

        self
    }

    #[doc(alias = "gdk_memory_texture_builder_set_update_region")]
    #[doc(alias = "update-region")]
    pub fn set_update_region(self, region: Option<&cairo::Region>) -> Self {
        unsafe {
            ffi::gdk_memory_texture_builder_set_update_region(
                self.to_glib_none().0,
                mut_override(region.to_glib_none().0),
            );
        }

        self
    }

    #[doc(alias = "gdk_memory_texture_builder_set_update_texture")]
    #[doc(alias = "update-texture")]
    pub fn set_update_texture(self, texture: Option<&impl IsA<Texture>>) -> Self {
        unsafe {
            ffi::gdk_memory_texture_builder_set_update_texture(
                self.to_glib_none().0,
                texture.map(|p| p.as_ref()).to_glib_none().0,
            );
        }

        self
    }

    #[doc(alias = "gdk_memory_texture_builder_set_width")]
    #[doc(alias = "width")]
    pub fn set_width(self, width: i32) -> Self {
        unsafe {
            ffi::gdk_memory_texture_builder_set_width(self.to_glib_none().0, width);
        }

        self
    }
}
