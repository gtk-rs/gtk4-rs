// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{prelude::*, Entry};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Entry>> Sealed for T {}
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Entry`](crate::Entry).
pub trait EntryExtManual: sealed::Sealed + IsA<Entry> + 'static {
    #[doc(alias = "gtk_entry_get_invisible_char")]
    #[doc(alias = "get_invisible_char")]
    fn invisible_char(&self) -> Option<char> {
        let ret =
            unsafe { crate::ffi::gtk_entry_get_invisible_char(self.as_ref().to_glib_none().0) };
        if ret == 0 {
            return None;
        }
        Some(TryFrom::try_from(ret).expect("conversion from an invalid Unicode value attempted"))
    }
}

impl<O: IsA<Entry>> EntryExtManual for O {}
