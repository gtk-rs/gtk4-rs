// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use gtk_sys;
use PadActionType;

#[repr(C)]
pub struct PadActionEntry(gtk_sys::GtkPadActionEntry);

impl PadActionEntry {
    pub fn new(type_: PadActionType, index: i32, mode: i32, label: &str,
               action_name: &str) -> PadActionEntry {
        assert_initialized_main_thread!();
        PadActionEntry(gtk_sys::GtkPadActionEntry {
            type_: type_.to_glib(),
            index,
            mode,
            label: label.to_glib_none().0,
            action_name: action_name.to_glib_none().0,
        })
    }

    pub fn get_type(&self) -> PadActionType {
        from_glib(self.0.type_)
    }

    pub fn get_index(&self) -> i32 {
        self.0.index
    }

    pub fn get_mode(&self) -> i32 {
        self.0.mode
    }

    pub fn get_label(&self) -> Option<String> {
        unsafe { from_glib_none(self.0.label) }
    }

    pub fn get_action_name(&self) -> Option<String> {
        unsafe { from_glib_none(self.0.label) }
    }
}

#[doc(hidden)]
impl ToGlib for PadActionEntry {
    type GlibType = gtk_sys::GtkPadActionEntry;

    fn to_glib(&self) -> gtk_sys::GtkPadActionEntry {
        self.0
    }
}
