// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Texture};
use glib::translate::*;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Texture`](crate::Texture).
pub trait TextureExtManual: 'static {
    #[doc(alias = "gdk_texture_download")]
    fn download(&self, data: &mut [u8], stride: usize);
}

impl<O: IsA<Texture>> TextureExtManual for O {
    fn download(&self, data: &mut [u8], stride: usize) {
        unsafe {
            assert!(
                stride >= 4,
                "Stride for a CAIRO_FORMAT_ARGB32 should be >= 4"
            );
            assert!(
                stride as i32 >= self.as_ref().width() * 4,
                "The stride must be >= 4*width"
            );
            assert!(
                data.len() as i32 >= stride as i32 * self.as_ref().height(),
                "The data is not big enough to download the texture"
            );
            ffi::gdk_texture_download(self.as_ref().to_glib_none().0, data.as_mut_ptr(), stride);
        }
    }
}
