// Copyright 2013-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>
/*

use gdk_sys;
use glib::translate::*;
#[repr(C)]
pub struct TimeCoord(gdk_sys::GdkTimeCoord);

impl TimeCoord {
    pub fn new(time: u32, axes: [f64; 128]) -> TimeCoord {
        assert_initialized_main_thread!();
        TimeCoord(gdk_sys::GdkTimeCoord { time, axes })
    }

    pub fn get_time(&self) -> u32 {
        self.0.time
    }
    pub fn get_axes(&self) -> &[f64; 128] {
        &self.0.axes
    }
}

#[doc(hidden)]
impl GlibPtrDefault for TimeCoord {
    type GlibType = *mut gdk_sys::GdkTimeCoord;
}
#[doc(hidden)]
impl FromGlibPtrNone<*mut gdk_sys::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_none(ptr: *mut gdk_sys::GdkTimeCoord) -> Self {
        TimeCoord(*ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gdk_sys::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_full(ptr: *mut gdk_sys::GdkTimeCoord) -> Self {
        let res = from_glib_none(ptr);
        glib_sys::g_free(ptr as *mut _);
        res
    }
}

#[doc(hidden)]
impl FromGlibContainerAsVec<gdk_sys::GdkTimeCoord, *mut gdk_sys::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut gdk_sys::GdkTimeCoord, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_none(ptr.add(i)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(
        ptr: *mut gdk_sys::GdkTimeCoord,
        num: usize,
    ) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut gdk_sys::GdkTimeCoord, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}
*/
