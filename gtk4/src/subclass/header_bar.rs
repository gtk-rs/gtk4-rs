use glib::subclass::prelude::*;

use super::widget::WidgetImpl;
use HeaderBarClass;
use WidgetClass;

pub trait HeaderBarImpl: WidgetImpl {}

unsafe impl<T: ObjectSubclass + HeaderBarImpl> IsSubclassable<T> for HeaderBarClass {
    fn override_vfuncs(&mut self) {
        <WidgetClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
