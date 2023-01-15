// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Native`](crate::Native) interface.

use crate::{subclass::prelude::*, Native};

pub trait NativeImpl: WidgetImpl {}

unsafe impl<T: NativeImpl> IsImplementable<T> for Native {}
