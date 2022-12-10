// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, DrawContext};
use glib::translate::*;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`DrawContext`](crate::DrawContext).
pub trait DrawContextExtManual: 'static {
    #[doc(alias = "gdk_draw_context_get_frame_region")]
    #[doc(alias = "get_frame_region")]
    fn frame_region(&self) -> Option<cairo::Region>;
}

impl<O: IsA<DrawContext>> DrawContextExtManual for O {
    fn frame_region(&self) -> Option<cairo::Region> {
        unsafe {
            from_glib_none(
                ffi::gdk_draw_context_get_frame_region(self.as_ref().to_glib_none().0)
                    as *mut cairo::ffi::cairo_region_t,
            )
        }
    }
}
