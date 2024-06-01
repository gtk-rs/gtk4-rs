// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::boxed::Box as Box_;

use crate::{ffi, CallbackAction, Widget};

impl CallbackAction {
    #[doc(alias = "gtk_callback_action_new")]
    pub fn new<P: Fn(&Widget, Option<&glib::Variant>) -> glib::Propagation + 'static>(
        callback: P,
    ) -> CallbackAction {
        assert_initialized_main_thread!();
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<
            P: Fn(&Widget, Option<&glib::Variant>) -> glib::Propagation + 'static,
        >(
            widget: *mut ffi::GtkWidget,
            args: *mut glib::ffi::GVariant,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let widget = from_glib_borrow(widget);
            let args: Borrowed<Option<glib::Variant>> = from_glib_borrow(args);
            let callback = &*(user_data as *mut P);
            (*callback)(&widget, args.as_ref().as_ref()).into_glib()
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn destroy_func<
            P: Fn(&Widget, Option<&glib::Variant>) -> glib::Propagation + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call2 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            from_glib_full(ffi::gtk_callback_action_new(
                callback,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call2,
            ))
        }
    }
}
