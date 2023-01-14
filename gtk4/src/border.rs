// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkBorder")]
    pub struct Border(BoxedInline<ffi::GtkBorder>);

    match fn {
        copy => |ptr| ffi::gtk_border_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_border_free(ptr),
        type_ => || ffi::gtk_border_get_type(),
    }
}

impl Border {
    #[doc(alias = "gtk_border_new")]
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { Self::uninitialized() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`Border`].
    ///
    /// This method returns an instance of [`BorderBuilder`] which can be used to create a [`Border`].
    pub fn builder() -> BorderBuilder {
        BorderBuilder::default()
    }

    #[inline]
    pub fn left(&self) -> i16 {
        self.inner.left
    }

    #[inline]
    pub fn set_left(&mut self, left: i16) {
        self.inner.left = left;
    }

    #[inline]
    pub fn right(&self) -> i16 {
        self.inner.right
    }

    #[inline]
    pub fn set_right(&mut self, right: i16) {
        self.inner.right = right;
    }

    #[inline]
    pub fn top(&self) -> i16 {
        self.inner.top
    }

    #[inline]
    pub fn set_top(&mut self, top: i16) {
        self.inner.top = top;
    }

    #[inline]
    pub fn bottom(&self) -> i16 {
        self.inner.bottom
    }

    #[inline]
    pub fn set_bottom(&mut self, bottom: i16) {
        self.inner.bottom = bottom;
    }
}

impl Default for Border {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Border {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.debug_struct("Border")
            .field("left", &self.left())
            .field("right", &self.right())
            .field("top", &self.top())
            .field("bottom", &self.bottom())
            .finish()
    }
}

impl PartialEq for Border {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.left() == other.left()
            && self.right() == other.right()
            && self.top() == other.top()
            && self.bottom() == other.bottom()
    }
}

impl Eq for Border {}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Border`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BorderBuilder {
    left: Option<i16>,
    right: Option<i16>,
    bottom: Option<i16>,
    top: Option<i16>,
}

impl BorderBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`BorderBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn left(mut self, left: i16) -> Self {
        self.left = Some(left);
        self
    }

    pub fn right(mut self, right: i16) -> Self {
        self.right = Some(right);
        self
    }

    pub fn bottom(mut self, bottom: i16) -> Self {
        self.bottom = Some(bottom);
        self
    }

    pub fn top(mut self, top: i16) -> Self {
        self.top = Some(top);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Border`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Border {
        let mut border = Border::default();
        if let Some(left) = self.left {
            border.set_left(left);
        }
        if let Some(right) = self.right {
            border.set_right(right);
        }
        if let Some(bottom) = self.bottom {
            border.set_bottom(bottom);
        }
        if let Some(top) = self.top {
            border.set_top(top);
        }
        border
    }
}
