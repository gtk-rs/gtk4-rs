// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::{DeleteEvent, EventType};

define_event! {
    DeleteEvent,
    crate::ffi::GdkDeleteEvent,
    &[EventType::Delete]
}

impl fmt::Debug for DeleteEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeleteEvent")
    }
}
