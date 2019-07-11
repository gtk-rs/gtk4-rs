use DrawContext;
use glib::IsA;
use glib::translate::*;

pub trait DrawContextExtManual: 'static {
    fn get_frame_region(&self) -> Option<cairo::Region>;
}

impl<O: IsA<DrawContext>> DrawContextExtManual for O {
    fn get_frame_region(&self) -> Option<cairo::Region> {
        unsafe {
            from_glib_none(gdk_sys::gdk_draw_context_get_frame_region(self.as_ref().to_glib_none().0) as *mut cairo_sys::cairo_region_t)
        }
    }
}
