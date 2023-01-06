// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, CellLayout, CellRenderer};
use glib::translate::*;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`CellLayout`](crate::CellLayout).
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait CellLayoutExtManual: 'static {
    #[doc(alias = "gtk_cell_layout_set_attributes")]
    fn set_attributes(&self, cell: &impl IsA<CellRenderer>, attributes: &[(&str, i32)]);

    #[doc(alias = "gtk_cell_layout_set_cell_data_func")]
    #[doc(alias = "set_cell_data_func")]
    fn unset_cell_data_func(&self, cell: &impl IsA<CellRenderer>);
}

impl<O: IsA<CellLayout>> CellLayoutExtManual for O {
    fn set_attributes(&self, cell: &impl IsA<CellRenderer>, attributes: &[(&str, i32)]) {
        self.as_ref().clear_attributes(cell);
        attributes.iter().for_each(|(attr, column)| {
            self.as_ref().add_attribute(cell, attr, *column);
        });
    }

    fn unset_cell_data_func(&self, cell: &impl IsA<CellRenderer>) {
        unsafe {
            ffi::gtk_cell_layout_set_cell_data_func(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            );
        }
    }
}
