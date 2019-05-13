// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use glib::object::{Cast, IsA};
use libc::c_char;
use std::ptr;
use FileChooserAction;
use FileChooserDialog;
use ResponseType;
use Widget;
use Window;

impl FileChooserDialog {
    pub fn new<T: IsA<Window>>(
        title: Option<&str>,
        parent: Option<&T>,
        action: FileChooserAction,
        buttons: &[(&str, ResponseType)]
    ) -> FileChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(match buttons.len() {
                0 => {
                    ffi::gtk_file_chooser_dialog_new(
                        title.to_glib_none().0,
                        parent.map(|p| p.as_ref()).to_glib_none().0,
                        action.to_glib(),
                        ptr::null::<c_char>()
                    )
                },
                1 => {
                    ffi::gtk_file_chooser_dialog_new(
                        title.to_glib_none().0,
                        parent.map(|p| p.as_ref()).to_glib_none().0,
                        action.to_glib(),
                        buttons[0].0.to_glib_none().0,
                        buttons[0].1.to_glib(),
                        ptr::null::<c_char>(),
                    )
                },
                2 => {
                    ffi::gtk_file_chooser_dialog_new(
                        title.to_glib_none().0,
                        parent.map(|p| p.as_ref()).to_glib_none().0,
                        action.to_glib(),
                        buttons[0].0.to_glib_none().0,
                        buttons[0].1.to_glib(),
                        (buttons[1].0.to_glib_none() as Stash<*const c_char, str>).0,
                        buttons[1].1.to_glib(),
                        ptr::null::<c_char>(),
                    )
                },
                3 => {
                    ffi::gtk_file_chooser_dialog_new(
                        title.to_glib_none().0,
                        parent.map(|p| p.as_ref()).to_glib_none().0,
                        action.to_glib(),
                        buttons[0].0.to_glib_none().0,
                        buttons[0].1.to_glib(),
                        (buttons[1].0.to_glib_none() as Stash<*const c_char, str>).0,
                        buttons[1].1.to_glib(),
                        (buttons[2].0.to_glib_none() as Stash<*const c_char, str>).0,
                        buttons[2].1.to_glib(),
                        ptr::null::<c_char>(),
                    )
                },
                _ => {
                    // TODO: Support arbitrary number of buttons once variadic functions are supported.
                    //       See: https://github.com/rust-lang/rust/issues/44930
                    panic!(format!("`FileChooserDialog::new` does not support 4+ buttons, received {}", buttons.len()))
                }
            }).unsafe_cast()
        }
    }
}
