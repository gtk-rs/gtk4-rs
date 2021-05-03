// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ConstraintGuide;
use glib::translate::*;
use glib::IsA;
use std::mem::MaybeUninit;

pub trait ConstraintGuideExtManual {
    #[doc(alias = "gtk_constraint_guide_get_max_size")]
    #[doc(alias = "get_max_size")]
    fn max_size(&self) -> (i32, i32);

    #[doc(alias = "gtk_constraint_guide_get_min_size")]
    #[doc(alias = "get_min_size")]
    fn min_size(&self) -> (i32, i32);

    #[doc(alias = "gtk_constraint_guide_get_nat_size")]
    #[doc(alias = "get_nat_size")]
    fn nat_size(&self) -> (i32, i32);
}

impl<O: IsA<ConstraintGuide>> ConstraintGuideExtManual for O {
    fn max_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();
            ffi::gtk_constraint_guide_get_max_size(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    fn min_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();
            ffi::gtk_constraint_guide_get_min_size(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    fn nat_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();
            ffi::gtk_constraint_guide_get_nat_size(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }
}
