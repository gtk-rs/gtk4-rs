// Take a look at the license at the top of the repository in the LICENSE file.

use crate::EditableLabel;

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::translate::*;
use glib::SignalHandlerId;
use std::boxed::Box as Box_;
use std::mem::transmute;

impl EditableLabel {
    #[doc(alias = "editing")]
    pub fn connect_editing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_editing_trampoline<F: Fn(&EditableLabel) + 'static>(
            this: *mut ffi::GtkEditableLabel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::editing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_editing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
