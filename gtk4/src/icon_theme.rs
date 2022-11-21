// Take a look at the license at the top of the repository in the LICENSE file.

use crate::IconTheme;
use glib::translate::*;

impl IconTheme {
    #[doc(alias = "gtk_icon_theme_get_icon_sizes")]
    #[doc(alias = "get_icon_sizes")]
    pub fn icon_sizes(&self, icon_name: &str) -> Vec<i32> {
        unsafe {
            let sizes_ptr = ffi::gtk_icon_theme_get_icon_sizes(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
            );
            let mut sizes = Vec::new();
            let mut i = 0;
            loop {
                // zero-terminated array
                let ret = std::ptr::read(sizes_ptr.add(i));
                if ret == 0 {
                    break;
                }
                sizes.push(ret);
                i += 1;
            }
            glib::ffi::g_free(sizes_ptr as *mut _);
            sizes
        }
    }
}
