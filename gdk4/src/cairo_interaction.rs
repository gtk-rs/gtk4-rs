// Copyright 2013-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use crate::{Rectangle, Surface, RGBA};
use cairo::{Context, Region};
use gdk_pixbuf::Pixbuf;
use glib::translate::*;

pub trait GdkCairoSurfaceExt {
    fn create_region(&self) -> Option<Region>;
}

impl GdkCairoSurfaceExt for cairo::Surface {
    fn create_region(&self) -> Option<Region> {
        unsafe {
            from_glib_full(ffi::gdk_cairo_region_create_from_surface(
                self.to_glib_none().0,
            ))
        }
    }
}

pub trait GdkCairoContextExt {
    /// # Safety
    ///
    /// It's the responsibility of the caller to ensure that source
    /// is a valid GL resource.
    unsafe fn draw_from_gl(
        &self,
        surface: &Surface,
        source: i32,
        source_type: i32,
        buffer_scale: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    );

    fn set_source_rgba(&self, rgba: &RGBA);

    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64);

    fn rectangle(&self, rectangle: &Rectangle);

    fn add_region(&self, region: &Region);
}

impl GdkCairoContextExt for Context {
    unsafe fn draw_from_gl(
        &self,
        surface: &Surface,
        source: i32,
        source_type: i32,
        buffer_scale: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        skip_assert_initialized!();
        ffi::gdk_cairo_draw_from_gl(
            mut_override(self.to_glib_none().0),
            surface.to_glib_none().0,
            source,
            source_type,
            buffer_scale,
            x,
            y,
            width,
            height,
        );
    }

    fn set_source_rgba(&self, rgba: &RGBA) {
        unsafe {
            ffi::gdk_cairo_set_source_rgba(self.to_glib_none().0, rgba.to_glib_none().0);
        }
    }

    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64) {
        unsafe {
            ffi::gdk_cairo_set_source_pixbuf(self.to_glib_none().0, pixbuf.to_glib_none().0, x, y);
        }
    }

    fn rectangle(&self, rectangle: &Rectangle) {
        unsafe {
            ffi::gdk_cairo_rectangle(self.to_glib_none().0, rectangle.to_glib_none().0);
        }
    }

    fn add_region(&self, region: &Region) {
        unsafe {
            ffi::gdk_cairo_region(self.to_glib_none().0, region.to_glib_none().0);
        }
    }
}
