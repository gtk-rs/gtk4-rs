use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use BinClass;

pub trait BinImpl: ContainerImpl + 'static {}

unsafe impl<T: ObjectSubclass + BinImpl> IsSubclassable<T> for BinClass {
    fn override_vfuncs(&mut self) {}
}
