// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CustomSorter, Ordering};
use glib::translate::*;
use std::ptr;

impl CustomSorter {
    #[doc(alias = "gtk_custom_sorter_new")]
    pub fn new<F>(sort_func: F) -> Self
    where
        F: Fn(&glib::Object, &glib::Object) -> Ordering + 'static,
    {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_custom_sorter_new(
                Some(trampoline::<F>),
                Box::into_raw(Box::new(sort_func)) as *mut _,
                Some(destroy_closure::<F>),
            ))
        }
    }

    #[doc(alias = "gtk_custom_sorter_set_sort_func")]
    pub fn set_sort_func<F>(&self, sort_func: F)
    where
        F: Fn(&glib::Object, &glib::Object) -> Ordering + 'static,
    {
        unsafe {
            ffi::gtk_custom_sorter_set_sort_func(
                self.to_glib_none().0,
                Some(trampoline::<F>),
                Box::into_raw(Box::new(sort_func)) as *mut _,
                Some(destroy_closure::<F>),
            )
        }
    }

    #[doc(alias = "gtk_custom_sorter_set_sort_func")]
    #[doc(alias = "set_sort_func")]
    pub fn unset_sort_func(&self) {
        unsafe {
            ffi::gtk_custom_sorter_set_sort_func(self.to_glib_none().0, None, ptr::null_mut(), None)
        }
    }
}

impl Default for CustomSorter {
    fn default() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_custom_sorter_new(None, ptr::null_mut(), None)) }
    }
}

unsafe extern "C" fn destroy_closure<F: Fn(&glib::Object, &glib::Object) -> Ordering + 'static>(
    ptr: glib::ffi::gpointer,
) {
    let _ = Box::<F>::from_raw(ptr as *mut _);
}

unsafe extern "C" fn trampoline<F: Fn(&glib::Object, &glib::Object) -> Ordering + 'static>(
    a: glib::ffi::gconstpointer,
    b: glib::ffi::gconstpointer,
    f: glib::ffi::gpointer,
) -> i32 {
    let f: &F = &*(f as *const F);
    f(
        &from_glib_borrow(a as *mut glib::gobject_ffi::GObject),
        &from_glib_borrow(b as *mut glib::gobject_ffi::GObject),
    )
    .into_glib()
}
