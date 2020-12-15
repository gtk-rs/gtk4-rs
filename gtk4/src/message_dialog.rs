// Take a look at the license at the top of the repository in the LICENSE file.

use glib::object::{Cast, IsA};
use glib::translate::*;
use libc::c_char;
use std::ptr;

use crate::{ButtonsType, DialogFlags, MessageDialog, MessageType, Widget, Window};

impl MessageDialog {
    pub fn new<T: IsA<Window>>(
        parent: Option<&T>,
        flags: DialogFlags,
        type_: MessageType,
        buttons: ButtonsType,
        message: &str,
    ) -> MessageDialog {
        assert_initialized_main_thread!();
        unsafe {
            let message: Stash<*const c_char, _> = message.to_glib_none();
            Widget::from_glib_none(ffi::gtk_message_dialog_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                type_.to_glib(),
                buttons.to_glib(),
                b"%s\0".as_ptr() as *const c_char,
                message.0,
                ptr::null::<c_char>(),
            ))
            .unsafe_cast()
        }
    }
}
