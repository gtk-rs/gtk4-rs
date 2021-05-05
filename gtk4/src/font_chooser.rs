// Take a look at the license at the top of the repository in the LICENSE file.

use crate::FontChooser;
use glib::translate::*;
use glib::IsA;

pub trait FontChooserExtManual: 'static {
    #[doc(alias = "gtk_font_chooser_set_filter_func")]
    #[doc(alias = "set_filter_func")]
    fn unset_filter_func(&self);
}

impl<O: IsA<FontChooser>> FontChooserExtManual for O {
    fn unset_filter_func(&self) {
        unsafe {
            ffi::gtk_font_chooser_set_filter_func(
                self.as_ref().to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            )
        }
    }
}
