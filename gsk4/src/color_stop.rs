// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::RGBA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskColorStop")]
    pub struct ColorStop(BoxedInline<ffi::GskColorStop>);
}

impl ColorStop {
    pub fn new(offset: f32, color: RGBA) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self::unsafe_from(ffi::GskColorStop {
                offset,
                color: *color.to_glib_none().0,
            })
        }
    }

    pub fn offset(&self) -> f32 {
        self.inner.offset
    }

    pub fn color(&self) -> &RGBA {
        unsafe { &*(&self.inner.color as *const gdk::ffi::GdkRGBA as *const RGBA) }
    }
}

impl fmt::Debug for ColorStop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ColorStop")
            .field("offset", &self.offset())
            .field("color", &self.color())
            .finish()
    }
}
