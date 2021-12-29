// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, MotionEvent};

define_event! {
    MotionEvent,
    ffi::GdkMotionEvent,
    &[EventType::MotionNotify]
}
