use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use HeaderBarClass;

pub trait HeaderBarImpl: ContainerImpl + 'static {}

unsafe impl<T: ObjectSubclass + HeaderBarImpl> IsSubclassable<T> for HeaderBarClass {
    fn override_vfuncs(&mut self) {}
}
