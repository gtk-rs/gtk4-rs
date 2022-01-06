// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Rectangle;
use cairo::RectangleInt;
use glib::translate::*;
use std::convert::{AsRef, From};
use std::fmt;

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        skip_assert_initialized!();
        unsafe {
            Self::unsafe_from(ffi::GdkRectangle {
                x,
                y,
                width,
                height,
            })
        }
    }

    pub fn x(&self) -> i32 {
        self.inner.x
    }

    pub fn y(&self) -> i32 {
        self.inner.y
    }

    pub fn width(&self) -> i32 {
        self.inner.width
    }

    pub fn height(&self) -> i32 {
        self.inner.height
    }
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Rectangle")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

impl AsRef<RectangleInt> for Rectangle {
    fn as_ref(&self) -> &RectangleInt {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl From<RectangleInt> for Rectangle {
    fn from(r: RectangleInt) -> Rectangle {
        skip_assert_initialized!();
        unsafe { *(&r as *const _ as *const _) }
    }
}
