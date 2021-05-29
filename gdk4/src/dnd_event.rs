// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Drop, EventType};
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct DNDEvent(Shared<ffi::GdkDNDEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

define_event! {
    DNDEvent,
    ffi::GdkDNDEvent,
    ffi::gdk_dnd_event_get_type,
    &[EventType::DragEnter, EventType::DragLeave, EventType::DragMotion, EventType::DropStart]
}

impl DNDEvent {
    #[doc(alias = "gdk_dnd_event_get_drop")]
    #[doc(alias = "get_drop")]
    pub fn drop(&self) -> Option<Drop> {
        unsafe { from_glib_none(ffi::gdk_dnd_event_get_drop(self.to_glib_none().0)) }
    }
}

impl fmt::Display for DNDEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DNDEvent")
            .field("drop", &self.drop())
            .finish()
    }
}
