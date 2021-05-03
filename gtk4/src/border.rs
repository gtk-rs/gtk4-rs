// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::fmt;
use std::ops;

glib::wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Border(Boxed<ffi::GtkBorder>);

    match fn {
        copy => |ptr| ffi::gtk_border_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_border_free(ptr),
        init => |_ptr| (),
        clear => |_ptr| (),
        type_ => || ffi::gtk_border_get_type(),
    }
}

impl ops::Deref for Border {
    type Target = ffi::GtkBorder;

    fn deref(&self) -> &Self::Target {
        &(*self.0)
    }
}

impl ops::DerefMut for Border {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut (*self.0)
    }
}

impl Border {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_border_new()) }
    }

    pub fn left(&self) -> i16 {
        self.left
    }

    pub fn set_left(&mut self, left: i16) {
        self.left = left;
    }

    pub fn right(&self) -> i16 {
        self.right
    }

    pub fn set_right(&mut self, right: i16) {
        self.right = right;
    }

    pub fn top(&self) -> i16 {
        self.top
    }

    pub fn set_top(&mut self, top: i16) {
        self.top = top;
    }

    pub fn bottom(&self) -> i16 {
        self.bottom
    }

    pub fn set_bottom(&mut self, bottom: i16) {
        self.bottom = bottom;
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

#[derive(Clone, Default)]
pub struct BorderBuilder {
    left: Option<i16>,
    right: Option<i16>,
    bottom: Option<i16>,
    top: Option<i16>,
}

impl BorderBuilder {
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
