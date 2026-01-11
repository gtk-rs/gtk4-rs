// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{Scale, prelude::*};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Scale`](crate::Scale).
pub trait ScaleExtManual: IsA<Scale> + 'static {
    #[doc(alias = "gtk_scale_set_format_value_func")]
    #[doc(alias = "set_format_value_func")]
    fn unset_format_value_func(&self) {
        unsafe {
            crate::ffi::gtk_scale_set_format_value_func(
                self.as_ref().to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            );
        }
    }
}

impl<O: IsA<Scale>> ScaleExtManual for O {}
