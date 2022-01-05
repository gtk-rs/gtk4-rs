// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, TouchEvent};
use std::fmt;

define_event! {
    TouchEvent,
    ffi::GdkTouchEvent,
    &[EventType::TouchBegin, EventType::TouchUpdate, EventType::TouchEnd, EventType::TouchCancel]
}

impl fmt::Debug for TouchEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TouchEvent")
            .field("is_emulating_pointer", &self.is_emulating_pointer())
            .finish()
    }
}
