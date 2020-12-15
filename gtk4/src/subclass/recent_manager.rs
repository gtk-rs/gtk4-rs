// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RecentManager;
use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Object};

pub trait RecentManagerImpl: RecentManagerImplExt + ObjectImpl {
    fn changed(&self, recent_manager: &Self::Type) {
        self.parent_changed(recent_manager)
    }
}

pub trait RecentManagerImplExt: ObjectSubclass {
    fn parent_changed(&self, recent_manager: &Self::Type);
}

impl<T: RecentManagerImpl> RecentManagerImplExt for T {
    fn parent_changed(&self, recent_manager: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkRecentManagerClass;
            if let Some(f) = (*parent_class).changed {
                f(recent_manager
                    .unsafe_cast_ref::<RecentManager>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: RecentManagerImpl> IsSubclassable<T> for RecentManager {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.changed = Some(recent_manager_changed::<T>);
    }
}

unsafe extern "C" fn recent_manager_changed<T: RecentManagerImpl>(ptr: *mut ffi::GtkRecentManager) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RecentManager> = from_glib_borrow(ptr);

    imp.changed(wrap.unsafe_cast_ref())
}
