use crate::ffi;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::widget::WidgetImpl;
use crate::{FlowBoxChild, Widget};

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
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkFlowBoxChildClass;
            if let Some(f) = (*parent_class).activate {
                f(child.unsafe_cast_ref::<FlowBoxChild>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: FlowBoxChildImpl> IsSubclassable<T> for FlowBoxChild {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.activate = Some(child_activate::<T>);
    }
}

unsafe extern "C" fn child_activate<T: FlowBoxChildImpl>(ptr: *mut ffi::GtkFlowBoxChild) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<FlowBoxChild> = from_glib_borrow(ptr);

    imp.activate(wrap.unsafe_cast_ref())
}
