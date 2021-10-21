// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::Fixed;

pub trait FixedImpl: WidgetImpl {}

unsafe impl<T: FixedImpl> IsSubclassable<T> for Fixed {}
