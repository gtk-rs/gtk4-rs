// Take a look at the license at the top of the repository in the LICENSE file.

use crate::CellEditable;
use glib::translate::*;
use glib::IsA;

pub trait CellEditableExtManual {
    #[doc(alias = "gtk_cell_editable_start_editing")]
    fn start_editing(&self, event: Option<&impl AsRef<gdk::Event>>);
}

impl<O: IsA<CellEditable>> CellEditableExtManual for O {
    fn start_editing(&self, event: Option<&impl AsRef<gdk::Event>>) {
        unsafe {
            ffi::gtk_cell_editable_start_editing(
                self.as_ref().to_glib_none().0,
                event.map(|e| e.as_ref()).to_glib_none().0,
            );
        }
    }
}
