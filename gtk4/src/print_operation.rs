// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PrintOperation;
use glib::translate::*;

impl PrintOperation {
    #[doc(alias = "gtk_print_operation_get_error")]
    #[doc(alias = "get_error")]
    pub fn error(&self) -> Option<glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            ffi::gtk_print_operation_get_error(self.to_glib_none().0, &mut error);
            if error.is_null() {
                None
            } else {
                Some(from_glib_full(error))
            }
        }
    }
}
