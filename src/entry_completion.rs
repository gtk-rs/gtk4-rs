// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use glib::Cast;
use gtk_sys;
use Entry;
use EntryCompletion;
use Widget;

pub trait EntryCompletionExtManual: 'static {
    fn get_entry(&self) -> Option<Entry>;
}

impl<O: IsA<EntryCompletion>> EntryCompletionExtManual for O {
    fn get_entry(&self) -> Option<Entry> {
        unsafe {
            Option::<Widget>::from_glib_none(gtk_sys::gtk_entry_completion_get_entry(
                self.as_ref().to_glib_none().0,
            ))
            .map(|widget| {
                widget
                    .downcast()
                    .expect("Non-Entry widget received from get_entry method")
            })
        }
    }
}
