// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Root`](crate::Root) interface.

use crate::{subclass::prelude::*, Root};

pub trait RootImpl: NativeImpl {}

unsafe impl<T: RootImpl> IsImplementable<T> for Root {}
