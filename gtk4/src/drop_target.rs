// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, DropTarget};
use glib::{
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    value::FromValue,
    Slice, Type,
};
use std::{boxed::Box as Box_, mem::transmute};

impl DropTarget {
    #[doc(alias = "gtk_drop_target_set_gtypes")]
    pub fn set_types(&self, types: &[Type]) {
        let types: Vec<glib::ffi::GType> = types.iter().map(|t| t.into_glib()).collect();
        unsafe {
            ffi::gtk_drop_target_set_gtypes(
                self.to_glib_none().0,
                mut_override(types.as_ptr()),
                types.len(),
            )
        }
    }

    #[doc(alias = "gtk_drop_target_get_value")]
    #[doc(alias = "get_value")]
    // rustdoc-stripper-ignore-next
    /// Similar to [`Self::value`] but panics if the value is of a different type.
    pub fn value_as<V: for<'b> FromValue<'b> + 'static>(&self) -> Option<V> {
        self.value().map(|v| {
            v.get_owned::<V>()
                .expect("Failed to get value as this type")
        })
    }

    #[doc(alias = "gtk_drop_target_get_gtypes")]
    #[doc(alias = "get_gtypes")]
    pub fn types(&self) -> Slice<Type> {
        unsafe {
            let mut n_types = std::mem::MaybeUninit::uninit();
            let types =
                ffi::gtk_drop_target_get_gtypes(self.to_glib_none().0, n_types.as_mut_ptr());

            Slice::from_glib_none_num(types, n_types.assume_init() as _)
        }
    }

    pub fn connect_drop<F: Fn(&DropTarget, &glib::Value, f64, f64) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drop_trampoline<
            F: Fn(&DropTarget, &glib::Value, f64, f64) -> bool + 'static,
        >(
            this: *mut ffi::GtkDropTarget,
            value: *mut glib::gobject_ffi::GValue,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &*(value as *const glib::Value),
                x,
                y,
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
