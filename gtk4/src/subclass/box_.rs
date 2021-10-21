// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::Box;

pub trait BoxImpl: WidgetImpl {}

unsafe impl<T: BoxImpl> IsSubclassable<T> for Box {}
