// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::object::Cast;
use glib::translate::*;
use std::ptr;
use Dialog;
use DialogFlags;
use IsA;
use Widget;
use Window;
use auto::DialogExt;
use ResponseType;

impl Dialog {
    pub fn new_with_buttons<T: IsA<Window>>(
        title: Option<&str>,
        parent: Option<&T>,
        flags: DialogFlags,
        buttons: &[(&str, ResponseType)],
    ) -> Dialog {
        assert_initialized_main_thread!();
        let ret: Dialog = unsafe {
            Widget::from_glib_none(
                ffi::gtk_dialog_new_with_buttons(title.to_glib_none().0, parent.map(|p| p.as_ref()).to_glib_none().0,
                    flags.to_glib(), ptr::null_mut()))
                .unsafe_cast()
        };

        ret.add_buttons(buttons);
        ret
    }
}

pub trait DialogExtManual: 'static {
    fn add_buttons(&self, buttons: &[(&str, ResponseType)]);
    fn run(&self) -> ResponseType;
    fn get_response_for_widget<P: IsA<Widget>>(&self, widget: &P) -> ResponseType;
}

impl<O: IsA<Dialog>> DialogExtManual for O {
    fn add_buttons(&self, buttons: &[(&str, ResponseType)]) {
        for &(text, id) in buttons {
            //FIXME: self.add_button don't work on 1.8
            O::add_button(self, text, id);
        }
    }

    fn run(&self) -> ResponseType {
        unsafe {
            from_glib(ffi::gtk_dialog_run(self.as_ref().to_glib_none().0))
        }
    }

    fn get_response_for_widget<P: IsA<Widget>>(&self, widget: &P) -> ResponseType {
        unsafe {
            from_glib(ffi::gtk_dialog_get_response_for_widget(self.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0))
        }
    }
}
