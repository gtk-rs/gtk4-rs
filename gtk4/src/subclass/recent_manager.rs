// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`RecentManager`](crate::RecentManager).

use crate::{prelude::*, subclass::prelude::*, RecentManager};
use glib::translate::*;

pub trait RecentManagerImpl: RecentManagerImplExt + ObjectImpl {
    fn changed(&self) {
        self.parent_changed()
    }
}

pub trait RecentManagerImplExt: ObjectSubclass {
    fn parent_changed(&self);
}

impl<T: RecentManagerImpl> RecentManagerImplExt for T {
    fn parent_changed(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkRecentManagerClass;
            if let Some(f) = (*parent_class).changed {
                f(self
                    .obj()
                    .unsafe_cast_ref::<RecentManager>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: RecentManagerImpl> IsSubclassable<T> for RecentManager {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();

        let klass = class.as_mut();
        klass.changed = Some(recent_manager_changed::<T>);
    }
}

unsafe extern "C" fn recent_manager_changed<T: RecentManagerImpl>(ptr: *mut ffi::GtkRecentManager) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.changed()
}
