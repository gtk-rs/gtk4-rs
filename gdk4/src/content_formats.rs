// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ContentFormats;
use glib::translate::*;

impl ContentFormats {
    #[doc(alias = "gdk_content_formats_get_gtypes")]
    pub fn gtypes(&self) -> Vec<glib::Type> {
        unsafe {
            let mut n_types = std::mem::MaybeUninit::uninit();
            let types =
                ffi::gdk_content_formats_get_gtypes(self.to_glib_none().0, n_types.as_mut_ptr());

            FromGlibContainer::from_glib_container_num(types, n_types.assume_init() as usize)
        }
    }
}
