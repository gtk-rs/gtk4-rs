// Take a look at the license at the top of the repository in the LICENSE file.

use cairo::RectangleInt;
use glib::translate::*;
use glib::StaticType;
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
            let mut ret = Self::uninitialized();
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
    pub fn union(&self, other: &Rectangle) -> Self {
        unsafe {
            let mut ret = Self::uninitialized();
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
    fn from(r: RectangleInt) -> Self {
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

impl glib::value::ValueType for Rectangle {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Rectangle {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(
            glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GdkRectangle
        )
    }
}

impl glib::value::ToValue for Rectangle {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.to_glib_none().0 as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for Rectangle {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.to_glib_none().0 as *mut _,
            )
        }
        value
    }
}
