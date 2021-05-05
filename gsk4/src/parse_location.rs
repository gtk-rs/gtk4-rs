// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ParseLocation(ffi::GskParseLocation);

impl ParseLocation {
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
impl FromGlibPtrBorrow<*const ffi::GskParseLocation> for ParseLocation {
    unsafe fn from_glib_borrow(
        ptr: *const ffi::GskParseLocation,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const ParseLocation))
    }
}
