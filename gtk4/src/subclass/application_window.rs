// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`ApplicationWindow`].

use crate::{ApplicationWindow, prelude::*, subclass::prelude::*};

pub trait ApplicationWindowImpl:
    WindowImpl
    + ObjectSubclass<Type: IsA<ApplicationWindow> + IsA<gio::ActionGroup> + IsA<gio::ActionMap>>
    + 'static
{
}

pub trait ApplicationWindowImplExt: ApplicationWindowImpl {}

impl<T: ApplicationWindowImpl> ApplicationWindowImplExt for T {}

unsafe impl<T: ApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {}
