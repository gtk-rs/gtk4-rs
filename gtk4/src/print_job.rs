// Take a look at the license at the top of the repository in the LICENSE file.

use std::boxed::Box as Box_;

use glib::translate::*;

use crate::{ffi, PrintJob};

impl PrintJob {
    #[doc(alias = "gtk_print_job_send")]
    pub fn send<P: Fn(&PrintJob, Result<(), glib::Error>) + 'static>(&self, callback: P) {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: Fn(&PrintJob, Result<(), glib::Error>) + 'static>(
            print_job: *mut ffi::GtkPrintJob,
            user_data: glib::ffi::gpointer,
            error: *const glib::ffi::GError,
        ) {
            let print_job = from_glib_borrow(print_job);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_none(error))
            };
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&print_job, result);
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn dnotify_func<P>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(dnotify_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            ffi::gtk_print_job_send(
                self.to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }
}
