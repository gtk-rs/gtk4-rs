// Take a look at the license at the top of the repository in the LICENSE file.

use crate::CustomFilter;
use glib::translate::*;
use std::ptr;

impl CustomFilter {
    #[doc(alias = "gtk_custom_filter_new")]
    pub fn new<F>(filter_func: F) -> Self
    where
        F: Fn(&glib::Object) -> bool + 'static,
    {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_custom_filter_new(
                Some(trampoline::<F>),
                Box::into_raw(Box::new(filter_func)) as *mut _,
                Some(destroy_closure::<F>),
            ))
        }
    }

    #[doc(alias = "gtk_custom_filter_set_filter_func")]
    pub fn set_filter_func<F>(&self, filter_func: F)
    where
        F: Fn(&glib::Object) -> bool + 'static,
    {
        unsafe {
            ffi::gtk_custom_filter_set_filter_func(
                self.to_glib_none().0,
                Some(trampoline::<F>),
                Box::into_raw(Box::new(filter_func)) as *mut _,
                Some(destroy_closure::<F>),
            )
        }
    }

    #[doc(alias = "gtk_custom_filter_set_filter_func")]
    #[doc(alias = "set_filter_func")]
    pub fn unset_filter_func(&self) {
        unsafe {
            ffi::gtk_custom_filter_set_filter_func(
                self.to_glib_none().0,
                None,
                ptr::null_mut(),
                None,
            )
        }
    }
}

impl Default for CustomFilter {
    fn default() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_custom_filter_new(None, ptr::null_mut(), None)) }
    }
}

unsafe extern "C" fn destroy_closure<F: Fn(&glib::Object) -> bool + 'static>(
    ptr: glib::ffi::gpointer,
) {
    let _ = Box::<F>::from_raw(ptr as *mut _);
}

unsafe extern "C" fn trampoline<F: Fn(&glib::Object) -> bool + 'static>(
    item: *mut glib::gobject_ffi::GObject,
    f: glib::ffi::gpointer,
) -> glib::ffi::gboolean {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(item)).into_glib()
}
