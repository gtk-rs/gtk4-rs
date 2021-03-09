// Take a look at the license at the top of the repository in the LICENSE file.

use crate::FileChooser;
use glib::subclass::prelude::*;

pub trait FileChooserImpl: ObjectImpl {}

unsafe impl<T: FileChooserImpl> IsImplementable<T> for FileChooser {
    fn interface_init(_iface: &mut glib::Class<Self>) {}

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}
