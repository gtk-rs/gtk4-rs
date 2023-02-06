// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Rectangle, Surface, RGBA};
use cairo::{Context, Region};
use gdk_pixbuf::Pixbuf;
use glib::translate::*;

// rustdoc-stripper-ignore-next
/// Trait containing integration methods with [`cairo::Surface`].
pub trait GdkCairoSurfaceExt {
    #[doc(alias = "gdk_cairo_region_create_from_surface")]
    fn create_region(&self) -> Region;
}

impl GdkCairoSurfaceExt for cairo::Surface {
    fn create_region(&self) -> Region {
        unsafe {
            from_glib_full(ffi::gdk_cairo_region_create_from_surface(
                self.to_glib_none().0,
            ))
        }
    }
}

// rustdoc-stripper-ignore-next
/// Trait containing integration methods with [`cairo::Context`].
pub trait GdkCairoContextExt {
    // rustdoc-stripper-ignore-next
    /// # Safety
    ///
    /// It's the responsibility of the caller to ensure that source
    /// is a valid GL resource.
    #[doc(alias = "gdk_cairo_draw_from_gl")]
    #[allow(clippy::too_many_arguments)]
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

    #[doc(alias = "gdk_cairo_set_source_rgba")]
    fn set_source_rgba(&self, rgba: &RGBA);

    #[doc(alias = "gdk_cairo_set_source_pixbuf")]
    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64);

    #[doc(alias = "gdk_cairo_rectangle")]
    fn add_rectangle(&self, rectangle: &Rectangle);

    #[doc(alias = "gdk_cairo_region")]
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

    fn add_rectangle(&self, rectangle: &Rectangle) {
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
