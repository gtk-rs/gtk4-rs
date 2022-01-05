// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, ScrollEvent};
use std::fmt;

define_event! {
    ScrollEvent,
    ffi::GdkScrollEvent,
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
