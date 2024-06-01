// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::{CrossingEvent, EventType};

define_event! {
    CrossingEvent,
    crate::ffi::GdkCrossingEvent,
    &[EventType::EnterNotify, EventType::LeaveNotify]
}

impl fmt::Debug for CrossingEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PadEvent")
            .field("detail", &self.detail())
            .field("focus", &self.gets_focus())
            .field("mode", &self.mode())
            .finish()
    }
}
