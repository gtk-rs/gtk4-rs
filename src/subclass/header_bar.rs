use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use ContainerClass;
use HeaderBarClass;

pub trait HeaderBarImpl: ContainerImpl + 'static {}

unsafe impl<T: ObjectSubclass + HeaderBarImpl> IsSubclassable<T> for HeaderBarClass {
    fn override_vfuncs(&mut self) {
        <ContainerClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
