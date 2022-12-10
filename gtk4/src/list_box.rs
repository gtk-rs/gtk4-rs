// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ListBox, ListBoxRow, Ordering};
use glib::translate::*;
use std::{boxed::Box as Box_, ptr};

impl ListBox {
    #[doc(alias = "gtk_list_box_bind_model")]
    #[doc(alias = "bind_model")]
    pub fn unbind_model(&self) {
        unsafe {
            ffi::gtk_list_box_bind_model(
                self.to_glib_none().0,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                None,
            )
        }
    }

    #[doc(alias = "gtk_list_box_set_filter_func")]
    #[doc(alias = "set_filter_func")]
    pub fn unset_filter_func(&self) {
        unsafe {
            ffi::gtk_list_box_set_filter_func(self.to_glib_none().0, None, ptr::null_mut(), None)
        }
    }

    #[doc(alias = "gtk_list_box_set_header_func")]
    #[doc(alias = "set_header_func")]
    pub fn unset_header_func(&self) {
        unsafe {
            ffi::gtk_list_box_set_header_func(self.to_glib_none().0, None, ptr::null_mut(), None)
        }
    }

    #[doc(alias = "gtk_list_box_set_sort_func")]
    pub fn set_sort_func<P: Fn(&ListBoxRow, &ListBoxRow) -> Ordering + 'static>(
        &self,
        sort_func: P,
    ) {
        let sort_func_data: Box_<P> = Box_::new(sort_func);
        unsafe extern "C" fn sort_func_func<
            P: Fn(&ListBoxRow, &ListBoxRow) -> Ordering + 'static,
        >(
            row1: *mut ffi::GtkListBoxRow,
            row2: *mut ffi::GtkListBoxRow,
            user_data: glib::ffi::gpointer,
        ) -> libc::c_int {
            let row1 = from_glib_borrow(row1);
            let row2 = from_glib_borrow(row2);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&row1, &row2);
            res.into_glib()
        }
        let sort_func = Some(sort_func_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&ListBoxRow, &ListBoxRow) -> Ordering + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = sort_func_data;
        unsafe {
            ffi::gtk_list_box_set_sort_func(
                self.to_glib_none().0,
                sort_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gtk_list_box_set_sort_func")]
    #[doc(alias = "set_sort_func")]
    pub fn unset_sort_func(&self) {
        unsafe {
            ffi::gtk_list_box_set_sort_func(self.to_glib_none().0, None, ptr::null_mut(), None)
        }
    }
}
