// Take a look at the license at the top of the repository in the LICENSE file.

use glib::ffi::gconstpointer;
use glib::translate::*;
use glib::StaticType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::str::FromStr;

#[derive(Debug)]
pub struct RgbaParseError;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[allow(clippy::upper_case_acronyms)]
pub struct RGBA {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl RGBA {
    #[doc(alias = "gdk_rgba_is_opaque")]
    pub fn is_opaque(&self) -> bool {
        skip_assert_initialized!();
        unsafe { from_glib(ffi::gdk_rgba_is_opaque(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_rgba_is_clear")]
    pub fn is_clear(&self) -> bool {
        skip_assert_initialized!();
        unsafe { from_glib(ffi::gdk_rgba_is_clear(self.to_glib_none().0)) }
    }

    pub fn black() -> Self {
        skip_assert_initialized!();
        Self {
            red: 0f32,
            green: 0f32,
            blue: 0f32,
            alpha: 1f32,
        }
    }

    pub fn blue() -> Self {
        skip_assert_initialized!();
        Self {
            red: 0f32,
            green: 0f32,
            blue: 1f32,
            alpha: 1f32,
        }
    }

    pub fn green() -> Self {
        skip_assert_initialized!();
        Self {
            red: 0f32,
            green: 1f32,
            blue: 0f32,
            alpha: 1f32,
        }
    }

    pub fn red() -> Self {
        skip_assert_initialized!();
        Self {
            red: 1f32,
            green: 0f32,
            blue: 0f32,
            alpha: 1f32,
        }
    }

    pub fn white() -> Self {
        skip_assert_initialized!();
        Self {
            red: 1f32,
            green: 1f32,
            blue: 1f32,
            alpha: 1f32,
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
            let mut rgba = Self::uninitialized();
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

impl glib::value::ValueType for RGBA {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for RGBA {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(
            glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GdkRGBA
        )
    }
}

impl glib::value::ToValue for RGBA {
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

impl glib::value::ToValueOptional for RGBA {
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
