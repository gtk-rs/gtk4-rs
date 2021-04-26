// Take a look at the license at the top of the repository in the LICENSE file.

use crate::IconTheme;
use glib::translate::*;

impl IconTheme {
    #[doc(alias = "gtk_icon_theme_set_resource_path")]
    pub fn set_resource_path(&self, path: &str) {
        unsafe {
            ffi::gtk_icon_theme_set_resource_path(self.to_glib_none().0, path.as_ptr() as *const _);
        }
    }
}
