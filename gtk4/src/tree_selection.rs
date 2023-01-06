// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TreeSelection;
use glib::translate::*;

impl TreeSelection {
    #[doc(alias = "gtk_tree_selection_set_select_function")]
    #[doc(alias = "set_select_function")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn unset_select_function(&self) {
        unsafe {
            ffi::gtk_tree_selection_set_select_function(
                self.to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            );
        }
    }
}
