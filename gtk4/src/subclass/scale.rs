// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Scale`](crate::Scale).

use crate::{prelude::*, subclass::prelude::*, Scale};
use glib::translate::*;

pub trait ScaleImpl: ScaleImplExt + RangeImpl {
    #[doc(alias = "get_layout_offsets")]
    fn layout_offsets(&self) -> (i32, i32) {
        self.parent_layout_offsets()
    }
}

pub trait ScaleImplExt: ObjectSubclass {
    fn parent_layout_offsets(&self) -> (i32, i32);
}

impl<T: ScaleImpl> ScaleImplExt for T {
    fn parent_layout_offsets(&self) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkScaleClass;
            let mut x = 0;
            let mut y = 0;
            if let Some(f) = (*parent_class).get_layout_offsets {
                f(
                    self.obj().unsafe_cast_ref::<Scale>().to_glib_none().0,
                    &mut x,
                    &mut y,
                );
            }
            (x, y)
        }
    }
}

unsafe impl<T: ScaleImpl> IsSubclassable<T> for Scale {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

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
    let imp = instance.imp();

    let (x, y) = imp.layout_offsets();
    *x_ptr = x;
    *y_ptr = y;
}
