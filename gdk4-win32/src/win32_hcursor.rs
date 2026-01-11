// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::c_void, ptr::NonNull};

use glib::{translate::*, types::Pointee};

use crate::{HCURSOR, Win32HCursor, ffi, prelude::*};

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
                handle.0 as isize,
                destroyable.into_glib(),
            ))
        }
    }

    pub fn handle(&self) -> HCURSOR {
        let ptr: NonNull<Pointee> = ObjectExt::property(self, "handle");
        HCURSOR(ptr.as_ptr())
    }
}
