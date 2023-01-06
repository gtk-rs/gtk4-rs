// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, CellRenderer, TreeView, TreeViewColumn, TreeViewColumnSizing};
use glib::translate::*;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`TreeView`](crate::TreeView).
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait TreeViewExtManual: 'static {
    #[doc(alias = "gtk_tree_view_insert_column_with_attributes")]
    fn insert_column_with_attributes(
        &self,
        position: i32,
        title: &str,
        cell: &impl IsA<CellRenderer>,
        attributes: &[(&str, i32)],
    ) -> i32;

    #[doc(alias = "gtk_tree_view_set_row_separator_func")]
    #[doc(alias = "set_row_separator_func")]
    fn unset_row_separator_func(&self);
}

impl<O: IsA<TreeView>> TreeViewExtManual for O {
    fn insert_column_with_attributes(
        &self,
        position: i32,
        title: &str,
        cell: &impl IsA<CellRenderer>,
        attributes: &[(&str, i32)],
    ) -> i32 {
        let column = TreeViewColumn::new();
        if self.as_ref().is_fixed_height_mode() {
            column.set_sizing(TreeViewColumnSizing::Fixed);
        }
        column.set_title(title);
        column.pack_start(cell, true);
        attributes.iter().for_each(|(attribute, column_id)| {
            column.add_attribute(cell, attribute, *column_id);
        });
        self.as_ref().insert_column(&column, position)
    }

    fn unset_row_separator_func(&self) {
        unsafe {
            ffi::gtk_tree_view_set_row_separator_func(
                self.as_ref().to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            );
        }
    }
}
