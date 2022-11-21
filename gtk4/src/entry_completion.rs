// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Entry, EntryCompletion, Widget};
use glib::translate::*;
use glib::Cast;

impl EntryCompletion {
    #[doc(alias = "gtk_entry_completion_get_entry")]
    #[doc(alias = "get_entry")]
    pub fn entry(&self) -> Option<Entry> {
        unsafe {
            Option::<Widget>::from_glib_none(ffi::gtk_entry_completion_get_entry(
                self.to_glib_none().0,
            ))
            .map(|widget| {
                widget
                    .downcast()
                    .expect("Non-Entry widget received from get_entry method")
            })
        }
    }
}
