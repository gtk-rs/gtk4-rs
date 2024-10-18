// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{prelude::*, FontChooser};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of
/// [`FontChooser`](crate::FontChooser).
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait FontChooserExtManual: IsA<FontChooser> + 'static {
    #[doc(alias = "gtk_font_chooser_set_filter_func")]
    #[doc(alias = "set_filter_func")]
    fn unset_filter_func(&self) {
        unsafe {
            crate::ffi::gtk_font_chooser_set_filter_func(
                self.as_ref().to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            )
        }
    }
}

impl<O: IsA<FontChooser>> FontChooserExtManual for O {}
