use glib::subclass::prelude::*;

use super::widget::WidgetImpl;
use BoxClass;
use WidgetClass;

pub trait BoxImpl: WidgetImpl {}

unsafe impl<T: ObjectSubclass + BoxImpl> IsSubclassable<T> for BoxClass {
    fn override_vfuncs(&mut self) {
        <WidgetClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
