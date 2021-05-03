// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[repr(C)]
#[derive(Clone)]
pub struct CssLocation(ffi::GtkCssLocation);

impl CssLocation {
    pub fn new(
        bytes: usize,
        chars: usize,
        lines: usize,
        line_bytes: usize,
        line_chars: usize,
    ) -> Self {
        assert_initialized_main_thread!();
        Self(ffi::GtkCssLocation {
            bytes,
            chars,
            lines,
            line_bytes,
            line_chars,
        })
    }

    pub fn bytes(&self) -> usize {
        self.0.bytes
    }

    pub fn chars(&self) -> usize {
        self.0.chars
    }

    pub fn lines(&self) -> usize {
        self.0.lines
    }

    pub fn line_bytes(&self) -> usize {
        self.0.line_bytes
    }

    pub fn line_chars(&self) -> usize {
        self.0.line_chars
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GtkCssLocation> for CssLocation {
    unsafe fn from_glib_none(ptr: *const ffi::GtkCssLocation) -> Self {
        Self(*ptr)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GtkCssLocation> for CssLocation {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<*const ffi::GtkCssLocation, Self> {
        Stash(&self.0, self)
    }
}
