// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::RGBA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskShadow")]
    pub struct Shadow(BoxedInline<ffi::GskShadow>);
}

impl Shadow {
    pub fn new(color: RGBA, dx: f32, dy: f32, radius: f32) -> Self {
        assert_initialized_main_thread!();
        Self(ffi::GskShadow {
            color: unsafe { *color.to_glib_none().0 },
            dx,
            dy,
            radius,
        })
    }

    pub fn color(&self) -> RGBA {
        unsafe { from_glib_none(&self.0.color as *const _) }
    }

    pub fn dx(&self) -> f32 {
        self.0.dx
    }

    pub fn dy(&self) -> f32 {
        self.0.dy
    }

    pub fn radius(&self) -> f32 {
        self.0.radius
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
