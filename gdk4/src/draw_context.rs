// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DrawContext;
use glib::translate::*;
use glib::IsA;

pub trait DrawContextExtManual: 'static {
    fn get_frame_region(&self) -> Option<cairo::Region>;
}

impl<O: IsA<DrawContext>> DrawContextExtManual for O {
    fn get_frame_region(&self) -> Option<cairo::Region> {
        unsafe {
            from_glib_none(
                ffi::gdk_draw_context_get_frame_region(self.as_ref().to_glib_none().0)
                    as *mut cairo::ffi::cairo_region_t,
            )
        }
    }
}
