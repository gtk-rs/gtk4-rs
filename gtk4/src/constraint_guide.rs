// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ConstraintGuide;
use glib::translate::*;
use std::mem::MaybeUninit;

impl ConstraintGuide {
    #[doc(alias = "gtk_constraint_guide_get_max_size")]
    #[doc(alias = "get_max_size")]
    pub fn max_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();
            ffi::gtk_constraint_guide_get_max_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    #[doc(alias = "gtk_constraint_guide_get_min_size")]
    #[doc(alias = "get_min_size")]
    pub fn min_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();
            ffi::gtk_constraint_guide_get_min_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    #[doc(alias = "gtk_constraint_guide_get_nat_size")]
    #[doc(alias = "get_nat_size")]
    pub fn nat_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();
            ffi::gtk_constraint_guide_get_nat_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }
}
