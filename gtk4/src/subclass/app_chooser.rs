// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AppChooser;
use glib::subclass::prelude::*;

pub trait AppChooserImpl: ObjectImpl {}

unsafe impl<T: AppChooserImpl> IsImplementable<T> for AppChooser {
    fn interface_init(_iface: &mut glib::Class<Self>) {}
}
