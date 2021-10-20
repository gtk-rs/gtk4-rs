// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkCssLocation")]
    pub struct CssLocation(BoxedInline<ffi::GtkCssLocation>);
}

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
