// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::widget::WidgetImpl;
use crate::{Allocation, Frame, Widget};

pub trait FrameImpl: FrameImplExt + WidgetImpl {
    fn compute_child_allocation(&self, frame: &Self::Type) -> Allocation {
        self.parent_compute_child_allocation(frame)
    }
}

pub trait FrameImplExt: ObjectSubclass {
    fn parent_compute_child_allocation(&self, frame: &Self::Type) -> Allocation;
}

impl<T: FrameImpl> FrameImplExt for T {
    fn parent_compute_child_allocation(&self, frame: &Self::Type) -> Allocation {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkFrameClass;
            let f = (*parent_class)
                .compute_child_allocation
                .expect("No parent class impl for \"compute_child_allocation\"");
            let mut allocation = Allocation::uninitialized();
            f(
                frame.unsafe_cast_ref::<Frame>().to_glib_none().0,
                allocation.to_glib_none_mut().0,
            );
            allocation
        }
    }
}

unsafe impl<T: FrameImpl> IsSubclassable<T> for Frame {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.compute_child_allocation = Some(frame_compute_child_allocation::<T>);
    }
}

unsafe extern "C" fn frame_compute_child_allocation<T: FrameImpl>(
    ptr: *mut ffi::GtkFrame,
    allocationptr: *mut ffi::GtkAllocation,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Frame> = from_glib_borrow(ptr);

    let allocation = imp.compute_child_allocation(wrap.unsafe_cast_ref());
    *allocationptr = *allocation.to_glib_none().0;
}
