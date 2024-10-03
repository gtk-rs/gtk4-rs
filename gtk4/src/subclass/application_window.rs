// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing
//! [`ApplicationWindow`](crate::ApplicationWindow).

use crate::{prelude::*, subclass::prelude::*, ApplicationWindow};

pub trait ApplicationWindowImpl:
    WindowImpl + ObjectSubclass<Type: IsA<ApplicationWindow>> + 'static
{
}

unsafe impl<T: ApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {}
