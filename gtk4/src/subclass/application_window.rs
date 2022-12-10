// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`ApplicationWindow`](crate::ApplicationWindow).

use crate::{subclass::prelude::*, ApplicationWindow};

pub trait ApplicationWindowImpl: WindowImpl + 'static {}

unsafe impl<T: ApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {}
