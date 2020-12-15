// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Entry, EntryCompletion, Widget};
use glib::object::IsA;
use glib::translate::*;
use glib::Cast;

pub trait EntryCompletionExtManual: 'static {
    fn get_entry(&self) -> Option<Entry>;
}

impl<O: IsA<EntryCompletion>> EntryCompletionExtManual for O {
    fn get_entry(&self) -> Option<Entry> {
        unsafe {
            Option::<Widget>::from_glib_none(ffi::gtk_entry_completion_get_entry(
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
