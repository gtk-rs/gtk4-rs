// Take a look at the license at the top of the repository in the LICENSE file.

use crate::EventType;
use glib::translate::*;
use std::fmt;

define_event! {
    MotionEvent,
    ffi::GdkMotionEvent,
    ffi::gdk_motion_event_get_type,
    &[EventType::MotionNotify]
}

impl fmt::Display for MotionEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MotionEvent")
    }
}
