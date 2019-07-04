// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;
use Gravity;

#[derive(Debug, Clone)]
pub struct Geometry(gdk_sys::GdkGeometry);

impl Geometry {
    pub fn new(min_width: i32, min_height: i32, max_width: i32, max_height: i32, base_width: i32, base_height: i32, width_inc: i32, height_inc: i32, min_aspect: f64, max_aspect: f64, win_gravity: Gravity) -> Geometry {
        assert_initialized_main_thread!();
        Geometry(gdk_sys::GdkGeometry {
            min_width,
            min_height,
            max_width,
            max_height,
            base_width,
            base_height,
            width_inc,
            height_inc,
            min_aspect,
            max_aspect,
            win_gravity: win_gravity.to_glib(),
        })
    }

    pub fn get_min_width(&self) -> i32 {
        self.0.min_width
    }

    pub fn get_min_height(&self) -> i32 {
        self.0.min_height
    }

    pub fn get_max_width(&self) -> i32 {
        self.0.max_width
    }

    pub fn get_max_height(&self) -> i32 {
        self.0.max_height
    }

    pub fn get_base_width(&self) -> i32 {
        self.0.base_width
    }

    pub fn get_base_height(&self) -> i32 {
        self.0.base_height
    }

    pub fn get_width_inc(&self) -> i32 {
        self.0.width_inc
    }

    pub fn get_height_inc(&self) -> i32 {
        self.0.height_inc
    }

    pub fn get_min_aspect(&self) -> f64 {
        self.0.min_aspect
    }

    pub fn get_max_aspect(&self) -> f64 {
        self.0.max_aspect
    }

    pub fn get_win_gravity(&self) -> Gravity {
        from_glib(self.0.win_gravity)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const gdk_sys::GdkGeometry> for Geometry {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<*const gdk_sys::GdkGeometry, Self> {
        Stash(&self.0, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut gdk_sys::GdkGeometry> for Geometry {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut gdk_sys::GdkGeometry, Self> {
        StashMut(&mut self.0, self)
    }
}
