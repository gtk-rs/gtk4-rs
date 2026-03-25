// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::c_void;

use crate::{ffi, prelude::*};
use glib::translate::*;

#[repr(transparent)]
#[doc(alias = "GdkToplevelSize")]
pub struct ToplevelSize(pub(crate) std::ptr::NonNull<ffi::GdkToplevelSize>);

impl StaticType for ToplevelSize {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_toplevel_size_get_type()) }
    }
}

impl ToplevelSize {
    #[doc(alias = "gdk_toplevel_size_get_bounds")]
    #[doc(alias = "get_bounds")]
    pub fn bounds(&self) -> (i32, i32) {
        unsafe {
            let mut bounds_width = std::mem::MaybeUninit::uninit();
            let mut bounds_height = std::mem::MaybeUninit::uninit();

            ffi::gdk_toplevel_size_get_bounds(
                self.0.as_ptr(),
                bounds_width.as_mut_ptr(),
                bounds_height.as_mut_ptr(),
            );
            (bounds_width.assume_init(), bounds_height.assume_init())
        }
    }

    #[doc(alias = "gdk_toplevel_size_set_min_size")]
    pub fn set_min_size(&mut self, min_width: i32, min_height: i32) {
        unsafe {
            ffi::gdk_toplevel_size_set_min_size(self.0.as_mut(), min_width, min_height);
        }
    }

    #[doc(alias = "gdk_toplevel_size_set_shadow_width")]
    pub fn set_shadow_width(&mut self, left: i32, right: i32, top: i32, bottom: i32) {
        unsafe {
            ffi::gdk_toplevel_size_set_shadow_width(self.0.as_mut(), left, right, top, bottom);
        }
    }

    #[doc(alias = "gdk_toplevel_size_set_size")]
    pub fn set_size(&mut self, width: i32, height: i32) {
        unsafe {
            ffi::gdk_toplevel_size_set_size(self.0.as_mut(), width, height);
        }
    }
}

impl std::fmt::Debug for ToplevelSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ToplevelSize")
            .field("bounds", &self.bounds())
            .finish()
    }
}

unsafe impl<'a> glib::value::FromValue<'a> for ToplevelSize {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        unsafe {
            let ptr = glib::gobject_ffi::g_value_get_pointer(value.to_glib_none().0);
            debug_assert!(
                !ptr.is_null(),
                "ToplevelSize in glib::Value is a NULL pointer"
            );
            ToplevelSize(std::ptr::NonNull::new_unchecked(
                ptr as *mut ffi::_GdkToplevelSize,
            ))
        }
    }
}

impl ToValue for ToplevelSize {
    #[inline]
    fn to_value(&self) -> glib::Value {
        unsafe {
            let mut v = glib::Value::from_type(Self::static_type());
            glib::gobject_ffi::g_value_set_pointer(
                v.to_glib_none_mut().0,
                self.0.as_ptr() as *mut c_void,
            );
            v
        }
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
