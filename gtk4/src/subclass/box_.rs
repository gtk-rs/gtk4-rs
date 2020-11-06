use glib::subclass::prelude::*;

use super::widget::WidgetImpl;
use Box;
use Widget;

pub trait BoxImpl: WidgetImpl {}

unsafe impl<T: BoxImpl> IsSubclassable<T> for Box {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);
    }
}
