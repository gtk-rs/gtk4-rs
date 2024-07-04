// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::UnsafeFrom;
use gtk4_sys::GtkAccessibleTextRange;

glib::wrapper! {
    #[doc(alias = "GtkAccessibleTextRange")]
    pub struct AccessibleTextRange(BoxedInline<crate::ffi::GtkAccessibleTextRange>);
}

impl AccessibleTextRange {
    pub fn new(start: usize, length: usize) -> Self {
        unsafe { AccessibleTextRange::unsafe_from(GtkAccessibleTextRange { start, length }) }
    }

    pub fn start(&self) -> usize {
        self.inner.start
    }

    pub fn length(&self) -> usize {
        self.inner.length
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
