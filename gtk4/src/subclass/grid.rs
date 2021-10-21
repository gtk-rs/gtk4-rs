// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::Grid;

pub trait GridImpl: WidgetImpl {}

unsafe impl<T: GridImpl> IsSubclassable<T> for Grid {}
