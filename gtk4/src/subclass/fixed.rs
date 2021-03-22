// Take a look at the license at the top of the repository in the LICENSE file.

use super::widget::WidgetImpl;
use crate::{Fixed, Widget};
use glib::subclass::prelude::*;

pub trait FixedImpl: WidgetImpl {}

unsafe impl<T: FixedImpl> IsSubclassable<T> for Fixed {
    fn class_init(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Widget as IsSubclassable<T>>::instance_init(instance);
    }
}
