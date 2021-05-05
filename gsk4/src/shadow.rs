// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::RGBA;
use glib::translate::*;
use std::mem;
use std::ptr;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Shadow(ffi::GskShadow);

impl Shadow {
    pub fn new(color: RGBA, dx: f32, dy: f32, radius: f32) -> Self {
        assert_initialized_main_thread!();
        Self(ffi::GskShadow {
            color: unsafe { *color.to_glib_none().0 },
            dx,
            dy,
            radius,
        })
    }

    pub fn color(&self) -> RGBA {
        unsafe { from_glib_none(&self.0.color as *const _) }
    }

    pub fn dx(&self) -> f32 {
        self.0.dx
    }

    pub fn dy(&self) -> f32 {
        self.0.dy
    }

    pub fn radius(&self) -> f32 {
        self.0.radius
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GskShadow> for Shadow {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<*const ffi::GskShadow, Self> {
        let ptr: *const Shadow = &*self;
        Stash(ptr as *const ffi::GskShadow, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GskShadow> for Shadow {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GskShadow, Self> {
        let ptr: *mut Shadow = &mut *self;
        StashMut(ptr as *mut ffi::GskShadow, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GskShadow> for Shadow {
    unsafe fn from_glib_none(ptr: *const ffi::GskShadow) -> Self {
        *(ptr as *const Shadow)
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *const ffi::GskShadow> for Shadow {
    type Storage = &'a [Self];

    fn to_glib_none_from_slice(t: &'a [Self]) -> (*const ffi::GskShadow, &'a [Self]) {
        assert_initialized_main_thread!();
        (t.as_ptr() as *const _, t)
    }

    fn to_glib_container_from_slice(t: &'a [Self]) -> (*const ffi::GskShadow, &'a [Self]) {
        assert_initialized_main_thread!();
        (ToGlibContainerFromSlice::to_glib_full_from_slice(t), t)
    }

    fn to_glib_full_from_slice(t: &[Self]) -> *const ffi::GskShadow {
        assert_initialized_main_thread!();

        if t.is_empty() {
            return ptr::null_mut();
        }

        unsafe {
            let res = glib::ffi::g_malloc(mem::size_of::<ffi::GskShadow>() * t.len())
                as *mut ffi::GskShadow;
            ptr::copy_nonoverlapping(t.as_ptr() as *const _, res, t.len());
            res
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *mut ffi::GskShadow> for Shadow {
    type Storage = &'a [Self];

    fn to_glib_none_from_slice(t: &'a [Self]) -> (*mut ffi::GskShadow, &'a [Self]) {
        assert_initialized_main_thread!();
        (t.as_ptr() as *mut ffi::GskShadow, t)
    }

    fn to_glib_container_from_slice(t: &'a [Self]) -> (*mut ffi::GskShadow, &'a [Self]) {
        assert_initialized_main_thread!();
        (ToGlibContainerFromSlice::to_glib_full_from_slice(t), t)
    }

    fn to_glib_full_from_slice(t: &[Self]) -> *mut ffi::GskShadow {
        assert_initialized_main_thread!();
        if t.is_empty() {
            return ptr::null_mut();
        }

        unsafe {
            let res = glib::ffi::g_malloc(mem::size_of::<ffi::GskShadow>() * t.len())
                as *mut ffi::GskShadow;
            ptr::copy_nonoverlapping(t.as_ptr() as *const _, res, t.len());
            res
        }
    }
}
