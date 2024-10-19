// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Orientable`] interface.

use crate::{prelude::*, subclass::prelude::*, Orientable};

pub trait OrientableImpl: ObjectImpl + ObjectSubclass<Type: IsA<Orientable>> {}

unsafe impl<T: OrientableImpl> IsImplementable<T> for Orientable {}
