// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, ScrollDirection};
use glib::translate::*;
use std::fmt;
use std::mem;

define_event! {
    ScrollEvent,
    ffi::GdkScrollEvent,
    ffi::gdk_scroll_event_get_type,
    &[EventType::Scroll]
}

impl ScrollEvent {
    #[doc(alias = "gdk_scroll_event_get_deltas")]
    #[doc(alias = "get_deltas")]
    pub fn deltas(&self) -> (f64, f64) {
        unsafe {
            let mut delta_x = mem::MaybeUninit::uninit();
            let mut delta_y = mem::MaybeUninit::uninit();
            ffi::gdk_scroll_event_get_deltas(
                self.to_glib_none().0,
                delta_x.as_mut_ptr(),
                delta_y.as_mut_ptr(),
            );
            let delta_x = delta_x.assume_init();
            let delta_y = delta_y.assume_init();
            (delta_x, delta_y)
        }
    }

    #[doc(alias = "gdk_scroll_event_get_direction")]
    #[doc(alias = "get_direction")]
    pub fn direction(&self) -> ScrollDirection {
        unsafe { from_glib(ffi::gdk_scroll_event_get_direction(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_scroll_event_is_stop")]
    pub fn is_stop(&self) -> bool {
        unsafe { from_glib(ffi::gdk_scroll_event_is_stop(self.to_glib_none().0)) }
    }
}

impl fmt::Display for ScrollEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ScrollEvent")
            .field("deltas", &self.deltas())
            .field("direction", &self.direction())
            .field("is_stop", &self.is_stop())
            .finish()
    }
}
