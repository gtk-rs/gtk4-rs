// Take a look at the license at the top of the repository in the LICENSE file.

use crate::IconTheme;
use glib::{translate::*, IntoGStr, Slice};

impl IconTheme {
    #[doc(alias = "gtk_icon_theme_get_icon_sizes")]
    #[doc(alias = "get_icon_sizes")]
    pub fn icon_sizes(&self, icon_name: impl IntoGStr) -> Slice<i32> {
        unsafe {
            let sizes_ptr = icon_name.run_with_gstr(|icon_name| {
                ffi::gtk_icon_theme_get_icon_sizes(self.to_glib_none().0, icon_name.as_ptr())
            });
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
