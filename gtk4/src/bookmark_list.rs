// Take a look at the license at the top of the repository in the LICENSE file.

use crate::BookmarkList;
use glib::translate::*;

impl BookmarkList {
    #[doc(alias = "gtk_bookmark_list_get_io_priority")]
    #[doc(alias = "get_io_priority")]
    pub fn io_priority(&self) -> glib::Priority {
        unsafe {
            from_glib(ffi::gtk_bookmark_list_get_io_priority(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_bookmark_list_set_io_priority")]
    pub fn set_io_priority(&self, io_priority: glib::Priority) {
        unsafe {
            ffi::gtk_bookmark_list_set_io_priority(self.to_glib_none().0, io_priority.into_glib());
        }
    }
}
