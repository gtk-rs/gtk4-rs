// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TreePath;
use glib::{translate::*, Slice};

impl TreePath {
    #[doc(alias = "gtk_tree_path_get_indices_with_depth")]
    #[doc(alias = "get_indices")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn indices(&self) -> Slice<i32> {
        unsafe {
            let mut count = 0;
            let ptr = ffi::gtk_tree_path_get_indices_with_depth(
                mut_override(self.to_glib_none().0),
                &mut count,
            );
            Slice::from_glib_none_num(ptr, count as usize)
        }
    }
}
