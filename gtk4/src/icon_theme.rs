// Take a look at the license at the top of the repository in the LICENSE file.

use crate::IconTheme;
use glib::{translate::*, Slice};

impl IconTheme {
    #[doc(alias = "gtk_icon_theme_get_icon_sizes")]
    #[doc(alias = "get_icon_sizes")]
    pub fn icon_sizes(&self, icon_name: &str) -> Slice<i32> {
        unsafe {
            let sizes_ptr = ffi::gtk_icon_theme_get_icon_sizes(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
            );
            let mut len = 0;
            if !sizes_ptr.is_null() {
                while std::ptr::read(sizes_ptr.add(len)) != 0 {
                    len += 1;
                }
            }
            Slice::from_glib_full_num(sizes_ptr, len)
        }
    }
}
