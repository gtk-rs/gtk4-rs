// Take a look at the license at the top of the repository in the LICENSE file.

use cairo::{Context, Region};
use gdk_pixbuf::Pixbuf;
use glib::translate::*;

use crate::{ffi, Rectangle, Surface, RGBA};

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
pub trait GdkCairoContextExt: sealed::Sealed {
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
    ) {
        skip_assert_initialized!();
        ffi::gdk_cairo_draw_from_gl(
            self.to_raw(),
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

    #[doc(alias = "gdk_cairo_set_source_rgba")]
    #[doc(alias = "set_source_rgba")]
    fn set_source_color(&self, rgba: &RGBA) {
        unsafe {
            ffi::gdk_cairo_set_source_rgba(self.to_raw(), rgba.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_cairo_set_source_pixbuf")]
    fn set_source_pixbuf(&self, pixbuf: &Pixbuf, x: f64, y: f64) {
        unsafe {
            ffi::gdk_cairo_set_source_pixbuf(self.to_raw(), pixbuf.to_glib_none().0, x, y);
        }
    }

    #[doc(alias = "gdk_cairo_rectangle")]
    fn add_rectangle(&self, rectangle: &Rectangle) {
        unsafe {
            ffi::gdk_cairo_rectangle(self.to_raw(), rectangle.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_cairo_region")]
    fn add_region(&self, region: &Region) {
        unsafe {
            ffi::gdk_cairo_region(self.to_raw(), region.to_glib_none().0);
        }
    }
}

impl GdkCairoContextExt for Context {}

mod sealed {
    use cairo::{ffi::cairo_t, Context};

    pub trait Sealed {
        fn to_raw(&self) -> *mut cairo_t;
    }

    impl Sealed for Context {
        fn to_raw(&self) -> *mut cairo_t {
            self.to_raw_none()
        }
    }
}
