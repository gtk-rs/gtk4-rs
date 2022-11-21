// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, MotionEvent};
use std::fmt;

define_event! {
    MotionEvent,
    ffi::GdkMotionEvent,
    &[EventType::MotionNotify]
}

impl fmt::Debug for MotionEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MotionEvent")
    }
}
