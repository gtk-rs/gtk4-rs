// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ListBox;
use glib::translate::*;
use std::ptr;

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
    #[doc(alias = "set_sort_func")]
    pub fn unset_sort_func(&self) {
        unsafe {
            ffi::gtk_list_box_set_sort_func(self.to_glib_none().0, None, ptr::null_mut(), None)
        }
    }
}
