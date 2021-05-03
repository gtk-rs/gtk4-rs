// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, TouchpadGesturePhase};
use glib::translate::*;
use std::fmt;
use std::mem;

define_event! {
    TouchpadEvent,
    ffi::GdkTouchpadEvent,
    ffi::gdk_touchpad_event_get_type,
    &[EventType::TouchpadSwipe, EventType::TouchpadPinch]
}

impl TouchpadEvent {
    #[doc(alias = "gdk_touchpad_event_get_deltas")]
    #[doc(alias = "get_deltas")]
    pub fn deltas(&self) -> (f64, f64) {
        unsafe {
            let mut dx = mem::MaybeUninit::uninit();
            let mut dy = mem::MaybeUninit::uninit();
            ffi::gdk_touchpad_event_get_deltas(
                self.to_glib_none().0,
                dx.as_mut_ptr(),
                dy.as_mut_ptr(),
            );
            let dx = dx.assume_init();
            let dy = dy.assume_init();
            (dx, dy)
        }
    }

    #[doc(alias = "gdk_touchpad_event_get_gesture_phase")]
    #[doc(alias = "get_gesture_phase")]
    pub fn gesture_phase(&self) -> TouchpadGesturePhase {
        unsafe {
            from_glib(ffi::gdk_touchpad_event_get_gesture_phase(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_touchpad_event_get_n_fingers")]
    #[doc(alias = "get_n_fingers")]
    pub fn n_fingers(&self) -> u32 {
        unsafe { ffi::gdk_touchpad_event_get_n_fingers(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_touchpad_event_get_pinch_angle_delta")]
    #[doc(alias = "get_pinch_angle_delta")]
    pub fn pinch_angle_delta(&self) -> f64 {
        unsafe { ffi::gdk_touchpad_event_get_pinch_angle_delta(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_touchpad_event_get_pinch_scale")]
    #[doc(alias = "get_pinch_scale")]
    pub fn pinch_scale(&self) -> f64 {
        unsafe { ffi::gdk_touchpad_event_get_pinch_scale(self.to_glib_none().0) }
    }
}

impl fmt::Display for TouchpadEvent {
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
