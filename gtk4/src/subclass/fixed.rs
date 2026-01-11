// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Fixed`].

use crate::{Fixed, prelude::*, subclass::prelude::*};

pub trait FixedImpl: WidgetImpl + ObjectSubclass<Type: IsA<Fixed>> {}

unsafe impl<T: FixedImpl> IsSubclassable<T> for Fixed {}
