// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::{EventType, PadEvent};

define_event! {
    PadEvent,
    crate::ffi::GdkPadEvent,
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
