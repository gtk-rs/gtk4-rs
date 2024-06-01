// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::translate::*;
use libc::c_char;

use crate::{ffi, prelude::*, FileChooserAction, FileChooserDialog, ResponseType, Widget, Window};

impl FileChooserDialog {
    #[doc(alias = "gtk_file_chooser_dialog_new")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn new(
        title: impl IntoOptionalGStr,
        parent: Option<&impl IsA<Window>>,
        action: FileChooserAction,
        buttons: &[(&str, ResponseType)],
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let dialog: Self = title.run_with_gstr(|title| {
                Widget::from_glib_none(ffi::gtk_file_chooser_dialog_new(
                    title.to_glib_none().0,
                    parent.map(|p| p.as_ref()).to_glib_none().0,
                    action.into_glib(),
                    ptr::null::<c_char>(),
                ))
                .unsafe_cast()
            });
            dialog.add_buttons(buttons);
            dialog
        }
    }
}

impl Default for FileChooserDialog {
    fn default() -> Self {
        glib::Object::new()
    }
}
