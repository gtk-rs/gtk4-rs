// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, DrawingArea};
use glib::translate::*;
use std::{cell::RefCell, ptr};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`DrawingArea`](crate::DrawingArea).
pub trait DrawingAreaExtManual: 'static {
    #[doc(alias = "gtk_drawing_area_set_draw_func")]
    #[doc(alias = "set_draw_func")]
    fn unset_draw_func(&self);

    #[doc(alias = "gtk_drawing_area_set_draw_func")]
    fn set_draw_func<P: FnMut(&DrawingArea, &cairo::Context, i32, i32) + 'static>(
        &self,
        draw_func: P,
    );
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

    fn set_draw_func<P: FnMut(&DrawingArea, &cairo::Context, i32, i32) + 'static>(
        &self,
        draw_func: P,
    ) {
        unsafe extern "C" fn draw_func_func<
            P: FnMut(&DrawingArea, &cairo::Context, i32, i32) + 'static,
        >(
            drawing_area: *mut ffi::GtkDrawingArea,
            cr: *mut cairo::ffi::cairo_t,
            width: libc::c_int,
            height: libc::c_int,
            user_data: glib::ffi::gpointer,
        ) {
            let drawing_area = from_glib_borrow(drawing_area);
            let cr = from_glib_borrow(cr);
            let callback: &RefCell<P> = &*(user_data as *mut _);
            (callback.borrow_mut())(&drawing_area, &cr, width, height);
        }

        unsafe extern "C" fn destroy_func<
            P: FnMut(&DrawingArea, &cairo::Context, i32, i32) + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box<RefCell<P>> = Box::from_raw(data as *mut _);
        }

        let callback: Box<RefCell<P>> = Box::new(RefCell::new(draw_func));
        unsafe {
            ffi::gtk_drawing_area_set_draw_func(
                self.as_ref().to_glib_none().0,
                Some(draw_func_func::<P> as _),
                Box::into_raw(callback) as *mut _,
                Some(destroy_func::<P> as _),
            );
        }
    }
}
