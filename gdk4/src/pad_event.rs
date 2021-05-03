// Take a look at the license at the top of the repository in the LICENSE file.

use crate::EventType;
use glib::translate::*;
use std::fmt;
use std::mem;

define_event! {
    PadEvent,
    ffi::GdkPadEvent,
    ffi::gdk_pad_event_get_type,
    &[EventType::PadButtonPress, EventType::PadButtonRelease, EventType::PadRing, EventType::PadStrip, EventType::PadGroupMode]
}

impl PadEvent {
    #[doc(alias = "gdk_pad_event_get_axis_value")]
    #[doc(alias = "get_axis_value")]
    pub fn axis_value(&self) -> (u32, f64) {
        unsafe {
            let mut index = mem::MaybeUninit::uninit();
            let mut value = mem::MaybeUninit::uninit();
            ffi::gdk_pad_event_get_axis_value(
                self.to_glib_none().0,
                index.as_mut_ptr(),
                value.as_mut_ptr(),
            );
            let index = index.assume_init();
            let value = value.assume_init();
            (index, value)
        }
    }

    #[doc(alias = "gdk_pad_event_get_button")]
    #[doc(alias = "get_button")]
    pub fn button(&self) -> u32 {
        unsafe { ffi::gdk_pad_event_get_button(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_pad_event_get_group_mode")]
    #[doc(alias = "get_group_mode")]
    pub fn group_mode(&self) -> (u32, u32) {
        unsafe {
            let mut group = mem::MaybeUninit::uninit();
            let mut mode = mem::MaybeUninit::uninit();
            ffi::gdk_pad_event_get_group_mode(
                self.to_glib_none().0,
                group.as_mut_ptr(),
                mode.as_mut_ptr(),
            );
            let group = group.assume_init();
            let mode = mode.assume_init();
            (group, mode)
        }
    }
}

impl fmt::Display for PadEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PadEvent")
            .field("axis_value", &self.axis_value())
            .field("button", &self.button())
            .field("group_mode", &self.group_mode())
            .finish()
    }
}
