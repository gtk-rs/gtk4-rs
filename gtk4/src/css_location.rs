// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkCssLocation")]
    pub struct CssLocation(BoxedInline<ffi::GtkCssLocation>);
}

impl CssLocation {
    #[inline]
    pub fn new(
        bytes: usize,
        chars: usize,
        lines: usize,
        line_bytes: usize,
        line_chars: usize,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self::unsafe_from(ffi::GtkCssLocation {
                bytes,
                chars,
                lines,
                line_bytes,
                line_chars,
            })
        }
    }

    #[inline]
    pub fn bytes(&self) -> usize {
        self.inner.bytes
    }

    #[inline]
    pub fn chars(&self) -> usize {
        self.inner.chars
    }

    #[inline]
    pub fn lines(&self) -> usize {
        self.inner.lines
    }

    #[inline]
    pub fn line_bytes(&self) -> usize {
        self.inner.line_bytes
    }

    #[inline]
    pub fn line_chars(&self) -> usize {
        self.inner.line_chars
    }
}

impl fmt::Debug for CssLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CssLocation")
            .field("bytes", &self.bytes())
            .field("chars", &self.chars())
            .field("lines", &self.lines())
            .field("line_bytes", &self.line_bytes())
            .field("line_chars", &self.line_chars())
            .finish()
    }
}
