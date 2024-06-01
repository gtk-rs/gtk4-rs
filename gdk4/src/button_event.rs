// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::{ButtonEvent, EventType};

define_event! {
    ButtonEvent,
    crate::ffi::GdkButtonEvent,
    &[EventType::ButtonPress, EventType::ButtonRelease]
}

impl fmt::Debug for ButtonEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ButtonEvent")
            .field("button", &self.button())
            .finish()
    }
}
