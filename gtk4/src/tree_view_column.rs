// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CellRenderer, TreeViewColumn};
use glib::translate::*;
use glib::IsA;

impl TreeViewColumn {
    #[doc(alias = "gtk_tree_view_column_new_with_attributes")]
    #[doc(alias = "new_with_attributes")]
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

    #[doc(alias = "gtk_tree_view_column_add_attribute")]
    pub fn add_attribute(
        &self,
        cell_renderer: &impl IsA<CellRenderer>,
        attribute: &str,
        column: i32,
    ) {
        unsafe {
            ffi::gtk_tree_view_column_add_attribute(
                self.to_glib_none().0,
                cell_renderer.as_ref().to_glib_none().0,
                attribute.to_glib_none().0,
                column,
            )
        }
    }

    #[doc(alias = "gtk_tree_view_column_pack_end")]
    pub fn pack_end(&self, cell_renderer: &impl IsA<CellRenderer>, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_pack_end(
                self.to_glib_none().0,
                cell_renderer.as_ref().to_glib_none().0,
                expand.into_glib(),
            )
        }
    }

    #[doc(alias = "gtk_tree_view_column_pack_start")]
    pub fn pack_start(&self, cell_renderer: &impl IsA<CellRenderer>, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_pack_start(
                self.to_glib_none().0,
                cell_renderer.as_ref().to_glib_none().0,
                expand.into_glib(),
            )
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_attributes")]
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

    #[doc(alias = "gtk_tree_view_column_clear")]
    pub fn clear(&self) {
        unsafe {
            ffi::gtk_tree_view_column_clear(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tree_view_column_clear_attributes")]
    pub fn clear_attributes(&self, cell_renderer: &impl IsA<CellRenderer>) {
        unsafe {
            ffi::gtk_tree_view_column_clear_attributes(
                self.to_glib_none().0,
                cell_renderer.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_tree_view_column_set_cell_data_func")]
    #[doc(alias = "set_cell_data_func")]
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
