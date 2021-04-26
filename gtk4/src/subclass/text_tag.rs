// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::TextTag;
use glib::Object;

pub trait TextTagImpl: ObjectImpl {}

unsafe impl<T: TextTagImpl> IsSubclassable<T> for TextTag {
    fn class_init(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}
