// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TreePath;
use glib::translate::*;
use std::slice;

impl TreePath {
    #[doc(alias = "gtk_tree_path_get_indices_with_depth")]
    #[doc(alias = "get_indices")]
    pub fn indices(&self) -> Vec<i32> {
        unsafe {
            let mut count = 0;
            let ptr = ffi::gtk_tree_path_get_indices_with_depth(
                mut_override(self.to_glib_none().0),
                &mut count,
            );
            if ptr.is_null() || count == 0 {
                vec![]
            } else {
                slice::from_raw_parts(ptr, count as usize).to_owned()
            }
        }
    }
}
