// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::FlowBoxChild;
use glib::translate::*;
use glib::Cast;

pub trait FlowBoxChildImpl: FlowBoxChildImplExt + WidgetImpl {
    fn activate(&self, child: &Self::Type) {
        self.parent_activate(child)
    }
}

pub trait FlowBoxChildImplExt: ObjectSubclass {
    fn parent_activate(&self, child: &Self::Type);
}

impl<T: FlowBoxChildImpl> FlowBoxChildImplExt for T {
    fn parent_activate(&self, child: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkFlowBoxChildClass;
            if let Some(f) = (*parent_class).activate {
                f(child.unsafe_cast_ref::<FlowBoxChild>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: FlowBoxChildImpl> IsSubclassable<T> for FlowBoxChild {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.activate = Some(child_activate::<T>);
    }
}

unsafe extern "C" fn child_activate<T: FlowBoxChildImpl>(ptr: *mut ffi::GtkFlowBoxChild) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<FlowBoxChild> = from_glib_borrow(ptr);

    imp.activate(wrap.unsafe_cast_ref())
}
