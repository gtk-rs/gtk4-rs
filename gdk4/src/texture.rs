// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Texture;
use glib::translate::*;
use glib::IsA;

pub trait TextureExtManual: 'static {
    #[doc(alias = "gdk_texture_download")]
    fn download(&self, data: &mut [u8], stride: usize);
}

impl<O: IsA<Texture>> TextureExtManual for O {
    fn download(&self, data: &mut [u8], stride: usize) {
        unsafe {
            ffi::gdk_texture_download(self.as_ref().to_glib_none().0, data.as_mut_ptr(), stride);
        }
    }
}
