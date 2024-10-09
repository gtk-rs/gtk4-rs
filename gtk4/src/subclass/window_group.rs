// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`WindowGroup`](crate::WindowGroup).

use crate::{prelude::*, subclass::prelude::*, WindowGroup};

pub trait WindowGroupImpl: ObjectImpl + ObjectSubclass<Type: IsA<WindowGroup>> {}

unsafe impl<T: WindowGroupImpl> IsSubclassable<T> for WindowGroup {}
