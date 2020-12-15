// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::widget::WidgetImpl;
use crate::{Box, Widget};

pub trait BoxImpl: WidgetImpl {}

unsafe impl<T: BoxImpl> IsSubclassable<T> for Box {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);
    }
}
