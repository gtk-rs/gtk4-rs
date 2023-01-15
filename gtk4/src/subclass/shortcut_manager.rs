// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`ShortcutManager`](crate::ShortcutManager) interface.

use crate::{prelude::*, subclass::prelude::*, ShortcutController, ShortcutManager};
use glib::translate::*;

pub trait ShortcutManagerImpl: ObjectImpl {
    fn add_controller(&self, controller: &ShortcutController) {
        self.parent_add_controller(controller);
    }

    fn remove_controller(&self, controller: &ShortcutController) {
        self.parent_remove_controller(controller)
    }
}

pub trait ShortcutManagerImplExt: ObjectSubclass {
    fn parent_add_controller(&self, controller: &ShortcutController);
    fn parent_remove_controller(&self, controller: &ShortcutController);
}

impl<T: ShortcutManagerImpl> ShortcutManagerImplExt for T {
    fn parent_add_controller(&self, controller: &ShortcutController) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ShortcutManager>()
                as *const ffi::GtkShortcutManagerInterface;

            let func = (*parent_iface)
                .add_controller
                .expect("no parent \"add_controller\" implementation");

            func(
                self.obj()
                    .unsafe_cast_ref::<ShortcutManager>()
                    .to_glib_none()
                    .0,
                controller.to_glib_none().0,
            )
        }
    }

    fn parent_remove_controller(&self, controller: &ShortcutController) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ShortcutManager>()
                as *const ffi::GtkShortcutManagerInterface;

            let func = (*parent_iface)
                .remove_controller
                .expect("no parent \"remove_controller\" implementation");

            func(
                self.obj()
                    .unsafe_cast_ref::<ShortcutManager>()
                    .to_glib_none()
                    .0,
                controller.to_glib_none().0,
            )
        }
    }
}

unsafe impl<T: ShortcutManagerImpl> IsImplementable<T> for ShortcutManager {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        assert_initialized_main_thread!();

        iface.add_controller = Some(shortcut_manager_add_controller::<T>);
        iface.remove_controller = Some(shortcut_manager_remove_controller::<T>);
    }
}

unsafe extern "C" fn shortcut_manager_add_controller<T: ShortcutManagerImpl>(
    shortcut_manager: *mut ffi::GtkShortcutManager,
    controller: *mut ffi::GtkShortcutController,
) {
    let instance = &*(shortcut_manager as *mut T::Instance);
    let imp = instance.imp();

    imp.add_controller(&ShortcutController::from_glib_borrow(controller))
}

unsafe extern "C" fn shortcut_manager_remove_controller<T: ShortcutManagerImpl>(
    shortcut_manager: *mut ffi::GtkShortcutManager,
    controller: *mut ffi::GtkShortcutController,
) {
    let instance = &*(shortcut_manager as *mut T::Instance);
    let imp = instance.imp();

    imp.remove_controller(&ShortcutController::from_glib_borrow(controller))
}
