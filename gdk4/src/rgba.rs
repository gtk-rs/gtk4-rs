// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use crate::ffi;
use glib::ffi::gconstpointer;
use glib::translate::*;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::str::FromStr;

#[derive(Debug)]
pub struct RgbaParseError;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RGBA {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl RGBA {
    // TODO: is_clear, is_opaque

    pub fn black() -> RGBA {
        skip_assert_initialized!();
        RGBA {
            red: 0f64,
            green: 0f64,
            blue: 0f64,
            alpha: 1f64,
        }
    }

    pub fn blue() -> RGBA {
        skip_assert_initialized!();
        RGBA {
            red: 0f64,
            green: 0f64,
            blue: 1f64,
            alpha: 1f64,
        }
    }

    pub fn green() -> RGBA {
        skip_assert_initialized!();
        RGBA {
            red: 0f64,
            green: 1f64,
            blue: 0f64,
            alpha: 1f64,
        }
    }

    pub fn red() -> RGBA {
        skip_assert_initialized!();
        RGBA {
            red: 1f64,
            green: 0f64,
            blue: 0f64,
            alpha: 1f64,
        }
    }

    pub fn white() -> RGBA {
        skip_assert_initialized!();
        RGBA {
            red: 1f64,
            green: 1f64,
            blue: 1f64,
            alpha: 1f64,
        }
    }
}

impl fmt::Display for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string: glib::GString =
            unsafe { from_glib_full(ffi::gdk_rgba_to_string(self.to_glib_none().0)) };
        f.write_str(&string)
    }
}

impl FromStr for RGBA {
    type Err = RgbaParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();
        unsafe {
            let mut rgba = RGBA::uninitialized();
            if from_glib(ffi::gdk_rgba_parse(
                rgba.to_glib_none_mut().0,
                s.to_glib_none().0,
            )) {
                Ok(rgba)
            } else {
                Err(RgbaParseError)
            }
        }
    }
}

impl Hash for RGBA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash = unsafe { ffi::gdk_rgba_hash(self.to_glib_none().0 as gconstpointer) };
        state.write_u32(hash);
    }
}

impl PartialEq for RGBA {
    fn eq(&self, other: &RGBA) -> bool {
        unsafe {
            from_glib(ffi::gdk_rgba_equal(
                self.to_glib_none().0 as gconstpointer,
                other.to_glib_none().0 as gconstpointer,
            ))
        }
    }
}

impl Eq for RGBA {}

#[doc(hidden)]
impl Uninitialized for RGBA {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GdkRGBA> for RGBA {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GdkRGBA, Self> {
        let ptr: *const RGBA = &*self;
        Stash(ptr as *const ffi::GdkRGBA, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GdkRGBA> for RGBA {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GdkRGBA, Self> {
        let ptr: *mut RGBA = &mut *self;
        StashMut(ptr as *mut ffi::GdkRGBA, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GdkRGBA> for RGBA {
    unsafe fn from_glib_none(ptr: *const ffi::GdkRGBA) -> Self {
        *(ptr as *const RGBA)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GdkRGBA> for RGBA {
    unsafe fn from_glib_none(ptr: *mut ffi::GdkRGBA) -> Self {
        *(ptr as *mut RGBA)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::GdkRGBA> for RGBA {
    unsafe fn from_glib_borrow(ptr: *const ffi::GdkRGBA) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const RGBA))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::GdkRGBA> for RGBA {
    unsafe fn from_glib_borrow(ptr: *mut ffi::GdkRGBA) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut RGBA))
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GdkRGBA> for RGBA {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GdkRGBA) -> Self {
        let rgba = *(ptr as *mut RGBA);
        ffi::gdk_rgba_free(ptr);
        rgba
    }
}

impl glib::StaticType for RGBA {
    fn static_type() -> glib::types::Type {
        skip_assert_initialized!();
        unsafe { from_glib(ffi::gdk_rgba_get_type()) }
    }
}

impl<'a> glib::value::FromValueOptional<'a> for RGBA {
    unsafe fn from_value_optional(value: &'a glib::Value) -> Option<Self> {
        from_glib_full(
            glib::gobject_ffi::g_value_dup_boxed(value.to_glib_none().0) as *mut ffi::GdkRGBA
        )
    }
}

impl glib::value::SetValue for RGBA {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            this.to_glib_none().0 as gconstpointer,
        )
    }
}

impl glib::value::SetValueOptional for RGBA {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        glib::gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            this.to_glib_none().0 as gconstpointer,
        )
    }
}
