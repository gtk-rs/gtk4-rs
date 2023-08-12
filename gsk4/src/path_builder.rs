// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PathBuilder;
use glib::translate::*;

impl PathBuilder {
    #[doc(alias = "gsk_path_builder_add_cairo_path")]
    pub fn add_cairo_path(&self, path: &cairo::Path) {
        unsafe {
            ffi::gsk_path_builder_add_cairo_path(self.to_glib_none().0, path.as_ptr());
        }
    }
}
