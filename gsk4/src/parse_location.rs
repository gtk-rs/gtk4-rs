// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskParseLocation")]
    pub struct ParseLocation(BoxedInline<ffi::GskParseLocation>);
}

impl ParseLocation {
    pub fn bytes(&self) -> usize {
        self.inner.bytes
    }

    pub fn chars(&self) -> usize {
        self.inner.chars
    }

    pub fn lines(&self) -> usize {
        self.inner.lines
    }

    pub fn line_bytes(&self) -> usize {
        self.inner.line_bytes
    }

    pub fn line_chars(&self) -> usize {
        self.inner.line_chars
    }
}

impl fmt::Debug for ParseLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ParseLocation")
            .field("bytes", &self.bytes())
            .field("chars", &self.chars())
            .field("lines", &self.lines())
            .field("line_bytes", &self.line_bytes())
            .field("line_chars", &self.line_chars())
            .finish()
    }
}
