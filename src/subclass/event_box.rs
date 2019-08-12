use glib::subclass::prelude::*;

use super::bin::BinImpl;
use EventBoxClass;

pub trait EventBoxImpl: BinImpl + 'static {}

unsafe impl<T: ObjectSubclass + EventBoxImpl> IsSubclassable<T> for EventBoxClass {
    fn override_vfuncs(&mut self) {}
}
