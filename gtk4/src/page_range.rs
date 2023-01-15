// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkPageRange")]
    pub struct PageRange(BoxedInline<ffi::GtkPageRange>);
}

impl PageRange {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        skip_assert_initialized!();
        unsafe { Self::unsafe_from(ffi::GtkPageRange { start, end }) }
    }

    #[inline]
    pub fn start(&self) -> i32 {
        self.inner.start
    }

    #[inline]
    pub fn end(&self) -> i32 {
        self.inner.end
    }
}

impl fmt::Debug for PageRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PageRange")
            .field("start", &self.start())
            .field("end", &self.end())
            .finish()
    }
}
