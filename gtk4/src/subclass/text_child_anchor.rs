// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TextChildAnchor;
use glib::subclass::prelude::*;
use glib::{subclass::object::ObjectImpl, Object};

pub trait TextChildAnchorImpl: ObjectImpl {}

unsafe impl<T: TextChildAnchorImpl> IsSubclassable<T> for TextChildAnchor {
    fn class_init(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}
