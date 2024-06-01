// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{prelude::*, CellLayout, CellRenderer};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::CellLayout>> Sealed for T {}
}
// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of
/// [`CellLayout`](crate::CellLayout).
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait CellLayoutExtManual: sealed::Sealed + IsA<CellLayout> + 'static {
    #[doc(alias = "gtk_cell_layout_set_attributes")]
    fn set_attributes(&self, cell: &impl IsA<CellRenderer>, attributes: &[(&str, i32)]) {
        self.as_ref().clear_attributes(cell);
        attributes.iter().for_each(|(attr, column)| {
            self.as_ref().add_attribute(cell, attr, *column);
        });
    }

    #[doc(alias = "gtk_cell_layout_set_cell_data_func")]
    #[doc(alias = "set_cell_data_func")]
    fn unset_cell_data_func(&self, cell: &impl IsA<CellRenderer>) {
        unsafe {
            crate::ffi::gtk_cell_layout_set_cell_data_func(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            );
        }
    }
}

impl<O: IsA<CellLayout>> CellLayoutExtManual for O {}
