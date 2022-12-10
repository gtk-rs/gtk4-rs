// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`FlowBoxChild`](crate::FlowBoxChild).

use crate::{prelude::*, subclass::prelude::*, FlowBoxChild};
use glib::translate::*;

pub trait FlowBoxChildImpl: FlowBoxChildImplExt + WidgetImpl {
    fn activate(&self) {
        self.parent_activate()
    }
}

pub trait FlowBoxChildImplExt: ObjectSubclass {
    fn parent_activate(&self);
}

impl<T: FlowBoxChildImpl> FlowBoxChildImplExt for T {
    fn parent_activate(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkFlowBoxChildClass;
            if let Some(f) = (*parent_class).activate {
                f(self
                    .obj()
                    .unsafe_cast_ref::<FlowBoxChild>()
                    .to_glib_none()
                    .0)
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
    let imp = instance.imp();

    imp.activate()
}
