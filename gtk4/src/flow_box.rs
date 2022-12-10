// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{FlowBox, FlowBoxChild, Ordering};
use glib::translate::*;
use std::{boxed::Box as Box_, ptr};

impl FlowBox {
    #[doc(alias = "gtk_flow_box_bind_model")]
    #[doc(alias = "bind_model")]
    pub fn unbind_model(&self) {
        unsafe {
            ffi::gtk_flow_box_bind_model(
                self.to_glib_none().0,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                None,
            )
        }
    }

    #[doc(alias = "gtk_flow_box_set_filter_func")]
    #[doc(alias = "set_filter_func")]
    pub fn unset_filter_func(&self) {
        unsafe {
            ffi::gtk_flow_box_set_filter_func(self.to_glib_none().0, None, ptr::null_mut(), None)
        }
    }

    #[doc(alias = "gtk_flow_box_set_sort_func")]
    #[doc(alias = "set_sort_func")]
    pub fn unset_sort_func(&self) {
        unsafe {
            ffi::gtk_flow_box_set_sort_func(self.to_glib_none().0, None, ptr::null_mut(), None)
        }
    }

    #[doc(alias = "gtk_flow_box_set_sort_func")]
    pub fn set_sort_func<P: Fn(&FlowBoxChild, &FlowBoxChild) -> Ordering + 'static>(
        &self,
        sort_func: P,
    ) {
        let sort_func_data: Box_<P> = Box_::new(sort_func);
        unsafe extern "C" fn sort_func_func<
            P: Fn(&FlowBoxChild, &FlowBoxChild) -> Ordering + 'static,
        >(
            child1: *mut ffi::GtkFlowBoxChild,
            child2: *mut ffi::GtkFlowBoxChild,
            user_data: glib::ffi::gpointer,
        ) -> libc::c_int {
            let child1 = from_glib_borrow(child1);
            let child2 = from_glib_borrow(child2);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&child1, &child2);
            res.into_glib()
        }
        let sort_func = Some(sort_func_func::<P> as _);
        unsafe extern "C" fn destroy_func<
            P: Fn(&FlowBoxChild, &FlowBoxChild) -> Ordering + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = sort_func_data;
        unsafe {
            ffi::gtk_flow_box_set_sort_func(
                self.to_glib_none().0,
                sort_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }
}
