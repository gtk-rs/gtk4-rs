// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::ApplicationWindow;

pub trait ApplicationWindowImpl: WindowImpl + 'static {}

unsafe impl<T: ApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {}
