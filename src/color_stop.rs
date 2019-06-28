// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use gdk::RGBA;
use glib_sys;
use gsk_sys;
use std::mem;
use std::ptr;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct ColorStop(gsk_sys::GskColorStop);

impl ColorStop {
    pub fn new(offset: f64, color: RGBA) -> ColorStop {
        assert_initialized_main_thread!();
        ColorStop(gsk_sys::GskColorStop {
            offset,
            color: unsafe { *color.to_glib_none().0 },
        })
    }

    pub fn get_offset(&self) -> f64 {
        self.0.offset
    }

    pub fn get_color(&self) -> RGBA {
        unsafe { from_glib_none(&self.0.color as *const _) }
    }
}

#[doc(hidden)]
impl FromGlibContainerAsVec<gsk_sys::GskColorStop, *const gsk_sys::GskColorStop> for ColorStop {
    unsafe fn from_glib_none_num_as_vec(ptr: *const gsk_sys::GskColorStop, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(ColorStop(ptr::read(ptr.add(i))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(_: *const gsk_sys::GskColorStop, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }

    unsafe fn from_glib_full_num_as_vec(_: *const gsk_sys::GskColorStop, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *const gsk_sys::GskColorStop> for ColorStop {
    type Storage = &'a [Self];

    fn to_glib_none_from_slice(t: &'a [Self]) -> (*const gsk_sys::GskColorStop, &'a [Self]) {
        assert_initialized_main_thread!();
        (t.as_ptr() as *const _, t)
    }

    fn to_glib_container_from_slice(t: &'a [Self]) -> (*const gsk_sys::GskColorStop, &'a [Self]) {
        assert_initialized_main_thread!();
        (ToGlibContainerFromSlice::to_glib_full_from_slice(t), t)
    }

    fn to_glib_full_from_slice(t: &[Self]) -> *const gsk_sys::GskColorStop {
        assert_initialized_main_thread!();

        if t.len() == 0 {
            return ptr::null_mut();
        }

        unsafe {
            let res = glib_sys::g_malloc(mem::size_of::<gsk_sys::GskColorStop>() * t.len()) as *mut gsk_sys::GskColorStop;
            ptr::copy_nonoverlapping(t.as_ptr() as *const _, res, t.len());
            res
        }
    }
}
