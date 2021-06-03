// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Texture;
use glib::translate::*;

impl Texture {
    #[doc(alias = "gdk_texture_download")]
    pub fn download(&self, data: &mut [u8], stride: usize) {
        unsafe {
            ffi::gdk_texture_download(self.to_glib_none().0, data.as_mut_ptr(), stride);
        }
    }
}
