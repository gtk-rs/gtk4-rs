// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::{FileChooserAction, FileChooserDialog, ResponseType, Widget, Window};
use glib::object::{Cast, IsA};
use glib::translate::*;
use libc::c_char;
use std::ptr;

impl FileChooserDialog {
    #[doc(alias = "gtk_file_chooser_dialog_new")]
    pub fn new(
        title: Option<&str>,
        parent: Option<&impl IsA<Window>>,
        action: FileChooserAction,
        buttons: &[(&str, ResponseType)],
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let dialog: Self = Widget::from_glib_none(ffi::gtk_file_chooser_dialog_new(
                title.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                action.into_glib(),
                ptr::null::<c_char>(),
            ))
            .unsafe_cast();
            dialog.add_buttons(buttons);
            dialog
        }
    }
}

impl Default for FileChooserDialog {
    fn default() -> Self {
        glib::Object::new(&[])
    }
}
