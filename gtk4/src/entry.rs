// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Entry};
use glib::translate::*;
use std::convert::TryFrom;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Entry`](crate::Entry).
pub trait EntryExtManual: 'static {
    #[doc(alias = "gtk_entry_get_invisible_char")]
    #[doc(alias = "get_invisible_char")]
    fn invisible_char(&self) -> Option<char>;
}

impl<O: IsA<Entry>> EntryExtManual for O {
    fn invisible_char(&self) -> Option<char> {
        let ret = unsafe { ffi::gtk_entry_get_invisible_char(self.as_ref().to_glib_none().0) };
        if ret == 0 {
            return None;
        }
        Some(TryFrom::try_from(ret).expect("conversion from an invalid Unicode value attempted"))
    }
}
