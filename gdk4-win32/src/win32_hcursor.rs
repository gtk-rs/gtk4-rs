// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Win32HCursor, HCURSOR};
use glib::{translate::*, types::Pointee};
use std::ptr::NonNull;

impl Win32HCursor {
    #[doc(alias = "gdk_win32_hcursor_new")]
    pub fn new(
        display: &impl IsA<crate::Win32Display>,
        handle: HCURSOR,
        destroyable: bool,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_win32_hcursor_new(
                display.as_ref().to_glib_none().0,
                handle.0,
                destroyable.into_glib(),
            ))
        }
    }

    pub fn handle(&self) -> HCURSOR {
        let ptr: NonNull<Pointee> = glib::ObjectExt::property(self, "handle");
        HCURSOR(ptr.as_ptr() as _)
    }
}
