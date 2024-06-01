// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, prelude::*, ComboBox};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ComboBox>> Sealed for T {}
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of
/// [`ComboBox`](crate::ComboBox).
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait ComboBoxExtManual: sealed::Sealed + IsA<ComboBox> + 'static {
    #[doc(alias = "gtk_combo_box_set_row_separator_func")]
    #[doc(alias = "set_row_separator_func")]
    fn unset_row_separator_func(&self) {
        unsafe {
            ffi::gtk_combo_box_set_row_separator_func(
                self.as_ref().to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            );
        }
    }

    #[doc(alias = "gtk_combo_box_set_active")]
    fn set_active(&self, index_: Option<u32>) {
        let index_ = match index_ {
            Some(i) => i as _,
            None => -1,
        };
        unsafe {
            ffi::gtk_combo_box_set_active(self.as_ref().to_glib_none().0, index_);
        }
    }

    #[doc(alias = "gtk_combo_box_get_active")]
    #[doc(alias = "get_active")]
    fn active(&self) -> Option<u32> {
        match unsafe { ffi::gtk_combo_box_get_active(self.as_ref().to_glib_none().0) } {
            -1 => None,
            x => Some(x as _),
        }
    }
}

impl<O: IsA<ComboBox>> ComboBoxExtManual for O {}
