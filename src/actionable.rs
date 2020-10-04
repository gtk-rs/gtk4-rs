// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use libc::c_char;
use Actionable;

pub trait ActionableExtManual: 'static {
    fn set_action_target(&self, string: &str);
}

impl<O: IsA<Actionable>> ActionableExtManual for O {
    fn set_action_target(&self, string: &str) {
        let string: Stash<*const c_char, _> = string.to_glib_none();
        unsafe {
            gtk_sys::gtk_actionable_set_action_target(
                self.as_ref().to_glib_none().0,
                b"%s\0".as_ptr() as *const c_char,
                string.0,
            );
        }
    }
}
