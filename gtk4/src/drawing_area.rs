// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DrawingArea;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

pub trait DrawingAreaExtManual: 'static {
    #[doc(alias = "gtk_drawing_area_set_draw_func")]
    #[doc(alias = "set_draw_func")]
    fn unset_draw_func(&self);
}
impl<O: IsA<DrawingArea>> DrawingAreaExtManual for O {
    fn unset_draw_func(&self) {
        unsafe {
            ffi::gtk_drawing_area_set_draw_func(
                self.as_ref().to_glib_none().0,
                None,
                ptr::null_mut(),
                None,
            )
        }
    }
}
