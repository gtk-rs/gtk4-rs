// Take a look at the license at the top of the repository in the LICENSE file.

use crate::EventType;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkDeleteEvent")]
    pub struct DeleteEvent(Shared<ffi::GdkDeleteEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

define_event! {
    DeleteEvent,
    ffi::GdkDeleteEvent,
    ffi::gdk_delete_event_get_type,
    &[EventType::Delete]
}

impl fmt::Display for DeleteEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeleteEvent")
    }
}
