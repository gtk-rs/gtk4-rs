// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, MapListModel};
use glib::translate::*;

impl MapListModel {
    #[doc(alias = "gtk_map_list_model_new")]
    pub fn without_map_func(model: Option<&impl IsA<gio::ListModel>>) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_map_list_model_new(
                model.map(|p| p.as_ref()).to_glib_full(),
                None,
                std::ptr::null_mut(),
                None,
            ))
        }
    }

    #[doc(alias = "gtk_map_list_model_set_map_func")]
    #[doc(alias = "set_map_func")]
    pub fn unset_map_func(&self) {
        unsafe {
            ffi::gtk_map_list_model_set_map_func(
                self.to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            );
        }
    }
}
