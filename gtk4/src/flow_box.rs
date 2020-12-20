// Take a look at the license at the top of the repository in the LICENSE file.

use crate::FlowBox;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

pub trait FlowBoxExtManual: 'static {
    fn unbind_model(&self);
}

impl<O: IsA<FlowBox>> FlowBoxExtManual for O {
    fn unbind_model(&self) {
        unsafe {
            ffi::gtk_flow_box_bind_model(
                self.as_ref().to_glib_none().0,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                None,
            )
        }
    }
}
