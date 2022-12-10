// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Box`](crate::Box).

use crate::{subclass::prelude::*, Box};

pub trait BoxImpl: WidgetImpl {}

unsafe impl<T: BoxImpl> IsSubclassable<T> for Box {}
