// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::RGBA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskShadow")]
    pub struct Shadow(BoxedInline<ffi::GskShadow>);
}

impl Shadow {
    #[inline]
    pub fn new(color: RGBA, dx: f32, dy: f32, radius: f32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self::unsafe_from(ffi::GskShadow {
                color: *color.to_glib_none().0,
                dx,
                dy,
                radius,
            })
        }
    }

    #[inline]
    pub fn color(&self) -> &RGBA {
        unsafe { &*(&self.inner.color as *const gdk::ffi::GdkRGBA as *const RGBA) }
    }

    #[inline]
    pub fn dx(&self) -> f32 {
        self.inner.dx
    }

    #[inline]
    pub fn dy(&self) -> f32 {
        self.inner.dy
    }

    #[inline]
    pub fn radius(&self) -> f32 {
        self.inner.radius
    }
}

impl fmt::Debug for Shadow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Shadow")
            .field("color", &self.color())
            .field("dx", &self.dx())
            .field("dy", &self.dy())
            .field("radius", &self.radius())
            .finish()
    }
}
