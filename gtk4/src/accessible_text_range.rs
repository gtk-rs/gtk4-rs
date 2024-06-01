// Take a look at the license at the top of the repository in the LICENSE file.

#[derive(Copy, Clone)]
#[doc(alias = "GtkAccessibleTextRange")]
#[repr(transparent)]
pub struct AccessibleTextRange(crate::ffi::GtkAccessibleTextRange);

impl AccessibleTextRange {
    pub fn start(&self) -> usize {
        self.0.start
    }

    pub fn length(&self) -> usize {
        self.0.length
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
