// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PopupLayout;
use glib::translate::*;

impl PopupLayout {
    #[doc(alias = "gdk_popup_layout_get_offset")]
    #[doc(alias = "get_offset")]
    pub fn offset(&self) -> (i32, i32) {
        let mut dx = 0;
        let mut dy = 0;
        unsafe {
            ffi::gdk_popup_layout_get_offset(self.to_glib_none().0, &mut dx, &mut dy);
        }
        (dx, dy)
    }
}
