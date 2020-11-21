use crate::ffi;
use crate::ConstraintGuide;
use glib::translate::*;
use glib::IsA;

pub trait ConstraintGuideExtManual {
    fn get_max_size(&self, width: i32, height: i32);

    fn get_min_size(&self, width: i32, height: i32);

    fn get_nat_size(&self, width: i32, height: i32);
}

impl<O: IsA<ConstraintGuide>> ConstraintGuideExtManual for O {
    fn get_max_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_constraint_guide_get_max_size(
                self.as_ref().to_glib_none().0,
                width as *mut _,
                height as *mut _,
            );
        }
    }

    fn get_min_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_constraint_guide_get_min_size(
                self.as_ref().to_glib_none().0,
                width as *mut _,
                height as *mut _,
            );
        }
    }

    fn get_nat_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_constraint_guide_get_nat_size(
                self.as_ref().to_glib_none().0,
                width as *mut _,
                height as *mut _,
            );
        }
    }
}
