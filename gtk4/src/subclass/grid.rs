// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Grid`](crate::Grid).

use crate::{subclass::prelude::*, Grid};

pub trait GridImpl: WidgetImpl {}

unsafe impl<T: GridImpl> IsSubclassable<T> for Grid {}
