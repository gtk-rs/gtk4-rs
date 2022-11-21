// Take a look at the license at the top of the repository in the LICENSE file.

use crate::MenuButton;
use glib::translate::*;

impl MenuButton {
    #[doc(alias = "gtk_menu_button_set_create_popup_func")]
    #[doc(alias = "set_create_popup_func")]
    pub fn unset_create_popup_func(&self) {
        unsafe {
            ffi::gtk_menu_button_set_create_popup_func(
                self.to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            );
        }
    }
}
