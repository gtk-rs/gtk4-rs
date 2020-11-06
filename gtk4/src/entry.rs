use glib::translate::*;
use glib::IsA;
use std::convert::TryFrom;
use Entry;

pub trait EntryExtManual: 'static {
    fn get_invisible_char(&self) -> Option<char>;
}

impl<O: IsA<Entry>> EntryExtManual for O {
    fn get_invisible_char(&self) -> Option<char> {
        let ret = unsafe { gtk_sys::gtk_entry_get_invisible_char(self.as_ref().to_glib_none().0) };
        if ret == 0 {
            return None;
        }
        Some(TryFrom::try_from(ret).expect("conversion from an invalid Unicode value attempted"))
    }
}
