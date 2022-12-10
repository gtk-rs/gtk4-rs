// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Fixed`](crate::Fixed).

use crate::{subclass::prelude::*, Fixed};

pub trait FixedImpl: WidgetImpl {}

unsafe impl<T: FixedImpl> IsSubclassable<T> for Fixed {}
