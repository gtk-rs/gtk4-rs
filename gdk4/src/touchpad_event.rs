// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, TouchpadEvent};
use std::fmt;

define_event! {
    TouchpadEvent,
    ffi::GdkTouchpadEvent,
    &[
        EventType::TouchpadSwipe,
        EventType::TouchpadPinch,
        #[cfg(feature = "v4_8")]
        {
            EventType::TouchpadHold
        },
    ]
}

impl fmt::Debug for TouchpadEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TouchpadEvent")
            .field("deltas", &self.deltas())
            .field("gesture_phase", &self.gesture_phase())
            .field("n_fingers", &self.n_fingers())
            .field("pinch_angle_delta", &self.pinch_angle_delta())
            .field("pinch_scale", &self.pinch_scale())
            .finish()
    }
}
