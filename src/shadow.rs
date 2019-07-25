// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk::RGBA;
use glib::translate::*;
use gsk_sys;

#[derive(Clone, Debug)]
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
        Stash(&self.0, self)
    }
}
