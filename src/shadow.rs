// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use gdk::RGBA;
use glib::translate::*;
use gsk_sys;
use std::mem;
use std::ptr;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Shadow(gsk_sys::GskShadow);

impl Shadow {
    pub fn new(color: RGBA, dx: f32, dy: f32, radius: f32) -> Shadow {
        assert_initialized_main_thread!();
        Shadow(gsk_sys::GskShadow {
            color: unsafe { *color.to_glib_none().0 },
            dx,
            dy,
            radius,
        })
    }

    pub fn get_color(&self) -> RGBA {
        unsafe { from_glib_none(&self.0.color as *const _) }
    }

    pub fn get_dx(&self) -> f32 {
        self.0.dx
    }

    pub fn get_dy(&self) -> f32 {
        self.0.dy
    }

    pub fn get_radius(&self) -> f32 {
        self.0.radius
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const gsk_sys::GskShadow> for Shadow {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<*const gsk_sys::GskShadow, Self> {
        let ptr: *const Shadow = &*self;
        Stash(ptr as *const gsk_sys::GskShadow, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut gsk_sys::GskShadow> for Shadow {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut gsk_sys::GskShadow, Self> {
        let ptr: *mut Shadow = &mut *self;
        StashMut(ptr as *mut gsk_sys::GskShadow, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const gsk_sys::GskShadow> for Shadow {
    unsafe fn from_glib_none(ptr: *const gsk_sys::GskShadow) -> Self {
        *(ptr as *const Shadow)
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *const gsk_sys::GskShadow> for Shadow {
    type Storage = &'a [Self];

    fn to_glib_none_from_slice(t: &'a [Self]) -> (*const gsk_sys::GskShadow, &'a [Self]) {
        assert_initialized_main_thread!();
        (t.as_ptr() as *const _, t)
    }

    fn to_glib_container_from_slice(t: &'a [Self]) -> (*const gsk_sys::GskShadow, &'a [Self]) {
        assert_initialized_main_thread!();
        (ToGlibContainerFromSlice::to_glib_full_from_slice(t), t)
    }

    fn to_glib_full_from_slice(t: &[Self]) -> *const gsk_sys::GskShadow {
        assert_initialized_main_thread!();

        if t.len() == 0 {
            return ptr::null_mut();
        }

        unsafe {
            let res = glib_sys::g_malloc(mem::size_of::<gsk_sys::GskShadow>() * t.len())
                as *mut gsk_sys::GskShadow;
            ptr::copy_nonoverlapping(t.as_ptr() as *const _, res, t.len());
            res
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *mut gsk_sys::GskShadow> for Shadow {
    type Storage = &'a [Self];

    fn to_glib_none_from_slice(t: &'a [Self]) -> (*mut gsk_sys::GskShadow, &'a [Self]) {
        assert_initialized_main_thread!();
        (t.as_ptr() as *mut gsk_sys::GskShadow, t)
    }

    fn to_glib_container_from_slice(t: &'a [Self]) -> (*mut gsk_sys::GskShadow, &'a [Self]) {
        assert_initialized_main_thread!();
        (ToGlibContainerFromSlice::to_glib_full_from_slice(t), t)
    }

    fn to_glib_full_from_slice(t: &[Self]) -> *mut gsk_sys::GskShadow {
        assert_initialized_main_thread!();
        if t.len() == 0 {
            return ptr::null_mut();
        }

        unsafe {
            let res = glib_sys::g_malloc(mem::size_of::<gsk_sys::GskShadow>() * t.len())
                as *mut gsk_sys::GskShadow;
            ptr::copy_nonoverlapping(t.as_ptr() as *const _, res, t.len());
            res
        }
    }
}
