use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use BoxClass;

pub trait BoxImpl: ContainerImpl + 'static {}

unsafe impl<T: ObjectSubclass + BoxImpl> IsSubclassable<T> for BoxClass {
    fn override_vfuncs(&mut self) {}
}
