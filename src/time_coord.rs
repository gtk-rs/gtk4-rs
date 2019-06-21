// Copyright 2013-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;

#[repr(C)]
#[derive(Clone)]
pub struct TimeCoord(ffi::GdkTimeCoord);

impl TimeCoord {
    pub fn new(time: u32, axes: [f64; 128]) -> TimeCoord {
        assert_initialized_main_thread!();
        TimeCoord(ffi::GdkTimeCoord {
            time,
            axes
        })
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
    type GlibType = *mut ffi::GdkTimeCoord;
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_none(ptr: *mut ffi::GdkTimeCoord) -> Self {
        TimeCoord((*ptr).clone())
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GdkTimeCoord> for TimeCoord {
    unsafe fn from_glib_full(ptr: *mut ffi::GdkTimeCoord) -> Self {
        let res = from_glib_none(ptr);
        glib_ffi::g_free(ptr as *mut _);
        res
    }
}
