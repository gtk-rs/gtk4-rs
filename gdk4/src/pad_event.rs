// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, PadEvent};
use std::fmt;

define_event! {
    PadEvent,
    ffi::GdkPadEvent,
    &[EventType::PadButtonPress, EventType::PadButtonRelease, EventType::PadRing, EventType::PadStrip, EventType::PadGroupMode]
}

impl fmt::Debug for PadEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PadEvent")
            .field("axis_value", &self.axis_value())
            .field("button", &self.button())
            .field("group_mode", &self.group_mode())
            .finish()
    }
}
