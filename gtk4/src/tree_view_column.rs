// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, CellRenderer, TreeViewColumn};
use glib::translate::*;

impl TreeViewColumn {
    #[doc(alias = "gtk_tree_view_column_new_with_attributes")]
    #[doc(alias = "new_with_attributes")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn with_attributes(
        title: &str,
        cell_renderer: &impl IsA<CellRenderer>,
        attributes: &[(&str, i32)],
    ) -> Self {
        assert_initialized_main_thread!();
        let tree_view_column = TreeViewColumn::new();
        tree_view_column.set_title(title);
        tree_view_column.pack_start(cell_renderer, true);
        tree_view_column.set_attributes(cell_renderer, attributes);

        tree_view_column
    }

    #[doc(alias = "gtk_tree_view_column_set_attributes")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn set_attributes(
        &self,
        cell_renderer: &impl IsA<CellRenderer>,
        attributes: &[(&str, i32)],
    ) {
        self.clear_attributes(cell_renderer);
        attributes.iter().for_each(|(attribute, column)| {
            self.add_attribute(cell_renderer, attribute, *column);
        });
    }

    #[doc(alias = "gtk_tree_view_column_set_cell_data_func")]
    #[doc(alias = "set_cell_data_func")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn unset_cell_data_func(&self, cell_renderer: &impl IsA<CellRenderer>) {
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
