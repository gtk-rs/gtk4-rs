// Take a look at the license at the top of the repository in the LICENSE file.

use crate::GestureStylus;
use gdk::AxisUse;
use glib::translate::*;

impl GestureStylus {
    #[doc(alias = "gtk_gesture_stylus_get_axes")]
    #[doc(alias = "get_axes")]
    pub fn axes(&self, axes: Vec<AxisUse>) -> Option<Vec<f64>> {
        let mut values = std::ptr::null_mut();
        unsafe {
            let mut axes1: Vec<gdk::ffi::GdkAxisUse> = axes.iter().map(|a| a.into_glib()).collect();
            axes1.push(gdk::ffi::GDK_AXIS_IGNORE);
            if from_glib(ffi::gtk_gesture_stylus_get_axes(
                self.to_glib_none().0,
                axes1.as_mut_ptr(),
                &mut values,
            )) {
                Some(FromGlibContainer::from_glib_container_num(
                    values,
                    axes.len(),
                ))
            } else {
                None
            }
        }
    }
}
