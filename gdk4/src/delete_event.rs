// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{DeleteEvent, EventType};
use std::fmt;

define_event! {
    DeleteEvent,
    ffi::GdkDeleteEvent,
    &[EventType::Delete]
}

impl fmt::Debug for DeleteEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeleteEvent")
    }
}
