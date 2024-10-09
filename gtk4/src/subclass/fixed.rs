// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Fixed`](crate::Fixed).

use crate::{prelude::*, subclass::prelude::*, Fixed};

pub trait FixedImpl: WidgetImpl + ObjectSubclass<Type: IsA<Fixed>> {}

unsafe impl<T: FixedImpl> IsSubclassable<T> for Fixed {}
