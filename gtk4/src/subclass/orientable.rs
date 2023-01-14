// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Orientable`](crate::Orientable) interface.

use crate::{subclass::prelude::*, Orientable};

pub trait OrientableImpl: WidgetImpl {}

unsafe impl<T: OrientableImpl> IsImplementable<T> for Orientable {}
