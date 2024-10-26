// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GtkAccessibleTextRange")]
    pub struct AccessibleTextRange(BoxedInline<ffi::GtkAccessibleTextRange>);
}

impl AccessibleTextRange {
    pub fn new(start: usize, length: usize) -> Self {
        skip_assert_initialized!();
        unsafe { AccessibleTextRange::unsafe_from(ffi::GtkAccessibleTextRange { start, length }) }
    }

    pub fn start(&self) -> usize {
        self.inner.start
    }

    pub fn set_start(&mut self, start: usize) {
        self.inner.start = start;
    }

    pub fn length(&self) -> usize {
        self.inner.length
    }

    pub fn set_length(&mut self, length: usize) {
        self.inner.length = length
    }
}

impl std::fmt::Debug for AccessibleTextRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccessibleTextRange")
            .field("start", &self.start())
            .field("length", &self.length())
            .finish()
    }
}
