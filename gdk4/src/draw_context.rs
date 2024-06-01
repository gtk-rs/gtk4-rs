// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{prelude::*, DrawContext};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DrawContext>> Sealed for T {}
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of
/// [`DrawContext`](crate::DrawContext).
pub trait DrawContextExtManual: sealed::Sealed + IsA<DrawContext> + 'static {
    #[doc(alias = "gdk_draw_context_get_frame_region")]
    #[doc(alias = "get_frame_region")]
    fn frame_region(&self) -> Option<cairo::Region> {
        unsafe {
            from_glib_none(crate::ffi::gdk_draw_context_get_frame_region(
                self.as_ref().to_glib_none().0,
            ) as *mut cairo::ffi::cairo_region_t)
        }
    }
}

impl<O: IsA<DrawContext>> DrawContextExtManual for O {}
