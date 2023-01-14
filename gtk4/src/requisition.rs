// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Requisition;
use std::fmt;

impl Requisition {
    #[inline]
    pub fn width(&self) -> i32 {
        self.inner.width
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.inner.height
    }
}

impl fmt::Debug for Requisition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Requisition")
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}
