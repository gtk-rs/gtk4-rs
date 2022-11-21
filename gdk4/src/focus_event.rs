// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, FocusEvent};
use std::fmt;

define_event! {
    FocusEvent,
    ffi::GdkFocusEvent,
    &[EventType::FocusChange]
}

impl fmt::Debug for FocusEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FocusEvent")
            .field("in", &self.is_in())
            .finish()
    }
}
