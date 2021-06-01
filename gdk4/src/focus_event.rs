// Take a look at the license at the top of the repository in the LICENSE file.

use crate::EventType;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkFocusEvent")]
    pub struct FocusEvent(Shared<ffi::GdkFocusEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

define_event! {
    FocusEvent,
    ffi::GdkFocusEvent,
    ffi::gdk_focus_event_get_type,
    &[EventType::FocusChange]
}

impl FocusEvent {
    #[doc(alias = "gdk_focus_event_get_in")]
    #[doc(alias = "get_in")]
    pub fn is_in(&self) -> bool {
        unsafe { from_glib(ffi::gdk_focus_event_get_in(self.to_glib_none().0)) }
    }
}

impl fmt::Display for FocusEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FocusEvent")
            .field("in", &self.is_in())
            .finish()
    }
}
