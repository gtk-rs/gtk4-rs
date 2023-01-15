// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Rectangle;
use cairo::RectangleInt;
use glib::translate::*;
use std::fmt;

impl Rectangle {
    #[inline]
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

    #[inline]
    pub fn x(&self) -> i32 {
        self.inner.x
    }

    #[inline]
    pub fn set_x(&mut self, x: i32) {
        self.inner.x = x;
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.inner.y
    }

    #[inline]
    pub fn set_y(&mut self, y: i32) {
        self.inner.y = y;
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.inner.width
    }

    #[inline]
    pub fn set_width(&mut self, width: i32) {
        self.inner.width = width;
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.inner.height
    }

    #[inline]
    pub fn set_height(&mut self, height: i32) {
        self.inner.height = height;
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
    #[inline]
    fn as_ref(&self) -> &RectangleInt {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl From<RectangleInt> for Rectangle {
    #[inline]
    fn from(r: RectangleInt) -> Self {
        skip_assert_initialized!();
        unsafe { *(&r as *const _ as *const _) }
    }
}

impl From<Rectangle> for RectangleInt {
    #[inline]
    fn from(r: Rectangle) -> Self {
        skip_assert_initialized!();
        unsafe { *(&r as *const _ as *const _) }
    }
}

impl From<cairo::Rectangle> for Rectangle {
    // rustdoc-stripper-ignore-next
    /// Note that converting between a [`cairo::Rectangle`] and [`Rectangle`]
    /// will probably lead to precisison errors. Use cautiously.
    #[inline]
    fn from(r: cairo::Rectangle) -> Self {
        skip_assert_initialized!();
        Self::new(
            r.x() as i32,
            r.y() as i32,
            r.width() as i32,
            r.height() as i32,
        )
    }
}

impl From<Rectangle> for cairo::Rectangle {
    #[inline]
    fn from(r: Rectangle) -> Self {
        skip_assert_initialized!();
        cairo::Rectangle::new(
            r.x() as f64,
            r.y() as f64,
            r.width() as f64,
            r.height() as f64,
        )
    }
}

impl From<pango::Rectangle> for Rectangle {
    #[inline]
    fn from(r: pango::Rectangle) -> Self {
        skip_assert_initialized!();
        Self::new(r.x(), r.y(), r.width(), r.height())
    }
}

impl From<Rectangle> for pango::Rectangle {
    #[inline]
    fn from(r: Rectangle) -> Self {
        skip_assert_initialized!();
        Self::new(r.x(), r.y(), r.width(), r.height())
    }
}
