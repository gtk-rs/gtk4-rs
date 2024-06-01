// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, Cursor, Texture};
use glib::translate::*;
use std::boxed::Box as Box_;

impl Cursor {
    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "gdk_cursor_new_from_callback")]
    #[doc(alias = "new_from_callback")]
    pub fn from_callback<
        P: Fn(&Cursor, i32, f64, &mut i32, &mut i32, &mut i32, &mut i32) -> Texture + 'static,
    >(
        callback: P,
        fallback: Option<&Cursor>,
    ) -> Option<Cursor> {
        assert_initialized_main_thread!();
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<
            P: Fn(&Cursor, i32, f64, &mut i32, &mut i32, &mut i32, &mut i32) -> Texture + 'static,
        >(
            cursor: *mut ffi::GdkCursor,
            cursor_size: libc::c_int,
            scale: libc::c_double,
            width: *mut libc::c_int,
            height: *mut libc::c_int,
            hotspot_x: *mut libc::c_int,
            hotspot_y: *mut libc::c_int,
            data: glib::ffi::gpointer,
        ) -> *mut ffi::GdkTexture {
            let cursor = from_glib_borrow(cursor);
            let callback = &*(data as *mut P);
            (*callback)(
                &cursor,
                cursor_size,
                scale,
                &mut *width,
                &mut *height,
                &mut *hotspot_x,
                &mut *hotspot_y,
            )
            /*Not checked*/
            .to_glib_none()
            .0
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn destroy_func<
            P: Fn(&Cursor, i32, f64, &mut i32, &mut i32, &mut i32, &mut i32) -> Texture + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call2 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            from_glib_full(ffi::gdk_cursor_new_from_callback(
                callback,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call2,
                fallback.to_glib_none().0,
            ))
        }
    }
}
