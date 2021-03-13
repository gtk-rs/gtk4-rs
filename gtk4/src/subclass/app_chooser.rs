// Take a look at the license at the top of the repository in the LICENSE file.

use super::widget::WidgetImpl;
use crate::AppChooser;
use glib::subclass::prelude::*;

pub trait AppChooserImpl: WidgetImpl {}

unsafe impl<T: AppChooserImpl> IsImplementable<T> for AppChooser {
    fn interface_init(_iface: &mut glib::Interface<Self>) {}

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}
