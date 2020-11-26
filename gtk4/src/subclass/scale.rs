// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::range::RangeImpl;
use crate::{Range, Scale};

pub trait ScaleImpl: ScaleImplExt + RangeImpl {
    fn get_layout_offsets(&self, scale: &Self::Type) -> (i32, i32) {
        self.parent_get_layout_offsets(scale)
    }
}

pub trait ScaleImplExt: ObjectSubclass {
    fn parent_get_layout_offsets(&self, scale: &Self::Type) -> (i32, i32);
}

impl<T: ScaleImpl> ScaleImplExt for T {
    fn parent_get_layout_offsets(&self, scale: &Self::Type) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkScaleClass;
            let mut x = 0;
            let mut y = 0;
            if let Some(f) = (*parent_class).get_layout_offsets {
                f(
                    scale.unsafe_cast_ref::<Scale>().to_glib_none().0,
                    &mut x,
                    &mut y,
                );
            }
            (x, y)
        }
    }
}

unsafe impl<T: ScaleImpl> IsSubclassable<T> for Scale {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Range as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.get_layout_offsets = Some(scale_get_layout_offsets::<T>);
    }
}

unsafe extern "C" fn scale_get_layout_offsets<T: ScaleImpl>(
    ptr: *mut ffi::GtkScale,
    x_ptr: *mut libc::c_int,
    y_ptr: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Scale> = from_glib_borrow(ptr);

    let (x, y) = imp.get_layout_offsets(wrap.unsafe_cast_ref());
    *x_ptr = x;
    *y_ptr = y;
}
