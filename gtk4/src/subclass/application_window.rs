// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::{ApplicationWindow, Window};

pub trait ApplicationWindowImpl: WindowImpl + 'static {}

unsafe impl<T: ApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {
    fn class_init(class: &mut glib::Class<Self>) {
        <Window as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Window as IsSubclassable<T>>::instance_init(instance);
    }
}
