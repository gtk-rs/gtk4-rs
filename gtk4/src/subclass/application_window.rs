use glib::subclass::prelude::*;

use super::window::WindowImpl;
use crate::{ApplicationWindow, Window};

pub trait ApplicationWindowImpl: WindowImpl + 'static {}

unsafe impl<T: ApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Window as IsSubclassable<T>>::override_vfuncs(class);
    }
}
