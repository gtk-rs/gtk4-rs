// Take a look at the license at the top of the repository in the LICENSE file.

use crate::IMContextSimple;
use glib::translate::*;
use std::path::Path;

impl IMContextSimple {
    #[doc(alias = "gtk_im_context_simple_add_compose_file")]
    pub fn add_compose_file(&self, compose_file: impl AsRef<Path>) {
        unsafe {
            let compose_file = compose_file.as_ref();
            ffi::gtk_im_context_simple_add_compose_file(
                self.to_glib_none().0,
                compose_file.to_glib_none().0,
            );
        }
    }
}
