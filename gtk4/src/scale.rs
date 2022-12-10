// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Scale};
use glib::translate::*;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Scale`](crate::Scale).
pub trait ScaleExtManual: 'static {
    #[doc(alias = "gtk_scale_set_format_value_func")]
    #[doc(alias = "set_format_value_func")]
    fn unset_format_value_func(&self);
}

impl<O: IsA<Scale>> ScaleExtManual for O {
    fn unset_format_value_func(&self) {
        unsafe {
            ffi::gtk_scale_set_format_value_func(
                self.as_ref().to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            );
        }
    }
}
