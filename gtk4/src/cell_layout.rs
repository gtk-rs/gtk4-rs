// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CellLayout, CellRenderer};
use glib::translate::*;
use glib::IsA;

pub trait CellLayoutExtManual: 'static {
    #[doc(alias = "gtk_cell_layout_set_cell_data_func")]
    #[doc(alias = "set_cell_data_func")]
    fn unset_cell_data_func<P: IsA<CellRenderer>>(&self, cell: &P);
}

impl<O: IsA<CellLayout>> CellLayoutExtManual for O {
    fn unset_cell_data_func<P: IsA<CellRenderer>>(&self, cell: &P) {
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
