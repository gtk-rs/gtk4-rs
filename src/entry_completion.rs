// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::Cast;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use Entry;
use EntryCompletion;
use Widget;

pub trait EntryCompletionExtManual: 'static {
    fn get_entry(&self) -> Option<Entry>;
}

impl<O: IsA<EntryCompletion>> EntryCompletionExtManual for O {
    fn get_entry(&self) -> Option<Entry> {
        let widget: Option<Widget> = unsafe {
            from_glib_none(gtk_sys::gtk_entry_completion_get_entry(self.as_ref().to_glib_none().0))
        };
        match widget {
            Some(w) => w.downcast().ok(),
            None => None,
        }
    }
}
