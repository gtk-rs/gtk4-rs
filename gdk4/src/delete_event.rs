// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{DeleteEvent, EventType};

define_event! {
    DeleteEvent,
    ffi::GdkDeleteEvent,
    &[EventType::Delete]
}
