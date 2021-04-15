// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CellRenderer, TreeViewColumn};
use glib::translate::*;
use glib::IsA;

impl TreeViewColumn {
    #[doc(alias = "gtk_tree_view_column_set_cell_data_func")]
    pub fn unset_cell_data_func<P: IsA<CellRenderer>>(&self, cell_renderer: &P) {
        unsafe {
            ffi::gtk_tree_view_column_set_cell_data_func(
                self.to_glib_none().0,
                cell_renderer.as_ref().to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            );
        }
    }
}
