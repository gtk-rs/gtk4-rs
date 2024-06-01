// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::{EventType, MotionEvent};

define_event! {
    MotionEvent,
    crate::ffi::GdkMotionEvent,
    &[EventType::MotionNotify]
}

impl fmt::Debug for MotionEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MotionEvent")
    }
}
