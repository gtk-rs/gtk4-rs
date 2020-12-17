// Take a look at the license at the top of the repository in the LICENSE file.

use cairo::RectangleInt;
use glib::ffi::gconstpointer;
use glib::translate::*;
use std::convert::{AsRef, From};
use std::mem;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    #[doc(alias = "gdk_rectangle_contains_point")]
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        unsafe {
            from_glib(ffi::gdk_rectangle_contains_point(
                self.to_glib_none().0,
                x,
                y,
            ))
        }
    }

    #[doc(alias = "gdk_rectangle_intersect")]
    pub fn intersect(&self, other: &Rectangle) -> Option<Rectangle> {
        unsafe {
            let mut ret = Rectangle::uninitialized();
            if from_glib(ffi::gdk_rectangle_intersect(
                self.to_glib_none().0,
                other.to_glib_none().0,
                ret.to_glib_none_mut().0,
            )) {
                Some(ret)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_rectangle_union")]
    pub fn union(&self, other: &Rectangle) -> Rectangle {
        unsafe {
            let mut ret = Rectangle::uninitialized();
            ffi::gdk_rectangle_union(
                self.to_glib_none().0,
                other.to_glib_none().0,
                ret.to_glib_none_mut().0,
            );
            ret
        }
    }
}

#[doc(hidden)]
impl Uninitialized for Rectangle {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GdkRectangle> for Rectangle {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GdkRectangle, Self> {
        let ptr: *const Rectangle = &*self;
        Stash(ptr as *const ffi::GdkRectangle, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GdkRectangle> for Rectangle {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GdkRectangle, Self> {
        let ptr: *mut Rectangle = &mut *self;
        StashMut(ptr as *mut ffi::GdkRectangle, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GdkRectangle> for Rectangle {
    unsafe fn from_glib_none(ptr: *const ffi::GdkRectangle) -> Self {
        *(ptr as *const Rectangle)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GdkRectangle> for Rectangle {
    unsafe fn from_glib_none(ptr: *mut ffi::GdkRectangle) -> Self {
        *(ptr as *mut Rectangle)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::GdkRectangle> for Rectangle {
    unsafe fn from_glib_borrow(ptr: *const ffi::GdkRectangle) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const Rectangle))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::GdkRectangle> for Rectangle {
    unsafe fn from_glib_borrow(ptr: *mut ffi::GdkRectangle) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut Rectangle))
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GdkRectangle> for Rectangle {
    unsafe fn from_glib_full(ptr: *mut ffi::GdkRectangle) -> Self {
        let rect = *(ptr as *mut Rectangle);
        glib::ffi::g_free(ptr as *mut _);
        rect
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const ffi::GdkRectangle> for Rectangle {
    unsafe fn from_glib_full(ptr: *const ffi::GdkRectangle) -> Self {
        let rect = *(ptr as *const Rectangle);
        glib::ffi::g_free(ptr as *mut _);
        rect
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

impl glib::StaticType for Rectangle {
    fn static_type() -> glib::types::Type {
        skip_assert_initialized!();
        unsafe { from_glib(ffi::gdk_rectangle_get_type()) }
    }
}

impl<'a> glib::value::FromValueOptional<'a> for Rectangle {
    unsafe fn from_value_optional(value: &'a glib::Value) -> Option<Self> {
        from_glib_full(
            glib::gobject_ffi::g_value_dup_boxed(value.to_glib_none().0) as *mut ffi::GdkRectangle
        )
    }
}

impl glib::value::SetValue for Rectangle {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            this.to_glib_none().0 as gconstpointer,
        )
    }
}

impl glib::value::SetValueOptional for Rectangle {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        glib::gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            this.to_glib_none().0 as gconstpointer,
        )
    }
}
