use glib::subclass::prelude::*;

use super::window::WindowImpl;
use ApplicationWindowClass;
use WindowClass;

pub trait ApplicationWindowImpl: WindowImpl + 'static {}

unsafe impl<T: ObjectSubclass + ApplicationWindowImpl> IsSubclassable<T>
    for ApplicationWindowClass
{
    fn override_vfuncs(&mut self) {
        <WindowClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
