// Copyright 2013-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use cairo::{Context, Region};
use gdk_pixbuf::Pixbuf;
use gdk_sys;
use glib::translate::*;
use {GLContext, Rectangle, Surface, RGBA};

pub trait GdkCairoSurfaceExt {
    fn create_region(&self) -> Option<Region>;
    unsafe fn upload_to_gl(
        &self,
        target: i32,
        width: i32,
        height: i32,
        context: Option<&GLContext>,
    );
}

impl GdkCairoSurfaceExt for cairo::Surface {
    fn create_region(&self) -> Option<Region> {
        unsafe {
            from_glib_full(gdk_sys::gdk_cairo_region_create_from_surface(
                self.to_glib_none().0,
            ))
        }
    }

    unsafe fn upload_to_gl(
        &self,
        target: i32,
        width: i32,
        height: i32,
        context: Option<&GLContext>,
    ) {
        assert_initialized_main_thread!();
        gdk_sys::gdk_cairo_surface_upload_to_gl(
            mut_override(self.to_glib_none().0),
            target,
            width,
            height,
            context.to_glib_none().0,
        );
    }
}

pub trait GdkCairoContextExt {
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

    fn get_clip_rectangle(&self) -> Option<Rectangle>;

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
        gdk_sys::gdk_cairo_draw_from_gl(
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

    fn get_clip_rectangle(&self) -> Option<Rectangle> {
        unsafe {
            let mut rectangle = Rectangle::uninitialized();
            if from_glib(gdk_sys::gdk_cairo_get_clip_rectangle(
                self.to_glib_none().0,
                rectangle.to_glib_none_mut().0,
            )) {
                Some(rectangle)
            } else {
                None
            }
        }
    }

    fn set_source_rgba(&self, rgba: &RGBA) {
        unsafe {
            gdk_sys::gdk_cairo_set_source_rgba(self.to_glib_none().0, rgba.to_glib_none().0);
        }
    }

    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64) {
        unsafe {
            gdk_sys::gdk_cairo_set_source_pixbuf(
                self.to_glib_none().0,
                pixbuf.to_glib_none().0,
                x,
                y,
            );
        }
    }

    fn rectangle(&self, rectangle: &Rectangle) {
        unsafe {
            gdk_sys::gdk_cairo_rectangle(self.to_glib_none().0, rectangle.to_glib_none().0);
        }
    }

    fn add_region(&self, region: &Region) {
        unsafe {
            gdk_sys::gdk_cairo_region(self.to_glib_none().0, region.to_glib_none().0);
        }
    }
}
