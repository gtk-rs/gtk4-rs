use glib::subclass::prelude::*;

use super::widget::WidgetImpl;
use HeaderBar;
use Widget;

pub trait HeaderBarImpl: WidgetImpl {}

unsafe impl<T: HeaderBarImpl> IsSubclassable<T> for HeaderBar {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);
    }
}
