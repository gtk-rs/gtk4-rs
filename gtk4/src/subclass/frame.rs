// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Frame`](crate::Frame).

use glib::translate::*;

use crate::{ffi, prelude::*, subclass::prelude::*, Allocation, Frame};

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait FrameImpl: FrameImplExt + WidgetImpl {
    fn compute_child_allocation(&self) -> Allocation {
        self.parent_compute_child_allocation()
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait FrameImplExt: ObjectSubclass {
    fn parent_compute_child_allocation(&self) -> Allocation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkFrameClass;
            let f = (*parent_class)
                .compute_child_allocation
                .expect("No parent class impl for \"compute_child_allocation\"");
            let mut allocation = Allocation::uninitialized();
            f(
                self.obj().unsafe_cast_ref::<Frame>().to_glib_none().0,
                allocation.to_glib_none_mut().0,
            );
            allocation
        }
    }
}

impl<T: FrameImpl> FrameImplExt for T {}

unsafe impl<T: FrameImpl> IsSubclassable<T> for Frame {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.compute_child_allocation = Some(frame_compute_child_allocation::<T>);
    }
}

unsafe extern "C" fn frame_compute_child_allocation<T: FrameImpl>(
    ptr: *mut ffi::GtkFrame,
    allocationptr: *mut ffi::GtkAllocation,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let allocation = imp.compute_child_allocation();
    *allocationptr = *allocation.to_glib_none().0;
}
