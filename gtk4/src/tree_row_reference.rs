// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, TreeIter, TreePath, TreeRowReference};
use glib::translate::*;

impl TreeRowReference {
    #[doc(alias = "gtk_tree_row_reference_reordered")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn reordered(
        &self,
        proxy: &impl IsA<glib::Object>,
        path: &TreePath,
        iter: &TreeIter,
        new_order: &[i32],
    ) {
        assert_eq!(
            new_order.len() as i32,
            self.model().iter_n_children(Some(iter)),
            "TreeRowReference got passed a `new_order` bigger than the total children in the model for the passed iter"
        );
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_tree_row_reference_reordered(
                proxy.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
                mut_override(iter.to_glib_none().0),
                mut_override(new_order.as_ptr()),
            )
        }
    }
}
