// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{DNDEvent, EventType};
use std::fmt;

define_event! {
    DNDEvent,
    ffi::GdkDNDEvent,
    &[EventType::DragEnter, EventType::DragLeave, EventType::DragMotion, EventType::DropStart]
}

impl fmt::Debug for DNDEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DNDEvent")
            .field("drop", &self.drop())
            .finish()
    }
}
