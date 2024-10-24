// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Grid`].

use crate::{prelude::*, subclass::prelude::*, Grid, Orientable};

pub trait GridImpl: WidgetImpl + ObjectSubclass<Type: IsA<Grid> + IsA<Orientable>> {}

unsafe impl<T: GridImpl> IsSubclassable<T> for Grid {}
