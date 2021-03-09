// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::widget::WidgetImpl;
use crate::{Box, Widget};

pub trait BoxImpl: WidgetImpl {}

unsafe impl<T: BoxImpl> IsSubclassable<T> for Box {
    fn class_init(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Widget as IsSubclassable<T>>::instance_init(instance);
    }
}
