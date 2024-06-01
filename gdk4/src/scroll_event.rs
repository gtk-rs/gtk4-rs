// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::{EventType, ScrollEvent};

define_event! {
    ScrollEvent,
    crate::ffi::GdkScrollEvent,
    &[EventType::Scroll]
}

impl fmt::Debug for ScrollEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ScrollEvent")
            .field("deltas", &self.deltas())
            .field("direction", &self.direction())
            .field("is_stop", &self.is_stop())
            .finish()
    }
}
