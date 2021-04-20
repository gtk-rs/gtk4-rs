// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ShortcutController, ShortcutManager};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait ShortcutManagerImpl: ObjectImpl {
    fn add_controller(&self, shortcut_manager: &Self::Type, controller: &ShortcutController) {
        self.parent_add_controller(shortcut_manager, controller);
    }

    fn remove_controller(&self, shortcut_manager: &Self::Type, controller: &ShortcutController) {
        self.parent_remove_controller(shortcut_manager, controller)
    }
}

pub trait ShortcutManagerImplExt: ObjectSubclass {
    fn parent_add_controller(&self, shortcut_manager: &Self::Type, controller: &ShortcutController);
    fn parent_remove_controller(
        &self,
        shortcut_manager: &Self::Type,
        controller: &ShortcutController,
    );
}

impl<T: ShortcutManagerImpl> ShortcutManagerImplExt for T {
    fn parent_add_controller(
        &self,
        shortcut_manager: &Self::Type,
        controller: &ShortcutController,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ShortcutManager>()
                as *const ffi::GtkShortcutManagerInterface;

            let func = (*parent_iface)
                .add_controller
                .expect("no parent \"add_controller\" implementation");

            func(
                shortcut_manager
                    .unsafe_cast_ref::<ShortcutManager>()
                    .to_glib_none()
                    .0,
                controller.to_glib_none().0,
            )
        }
    }

    fn parent_remove_controller(
        &self,
        shortcut_manager: &Self::Type,
        controller: &ShortcutController,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ShortcutManager>()
                as *const ffi::GtkShortcutManagerInterface;

            let func = (*parent_iface)
                .remove_controller
                .expect("no parent \"remove_controller\" implementation");

            func(
                shortcut_manager
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

        iface.add_controller = Some(shortcut_manager_add_controller::<T>);
        iface.remove_controller = Some(shortcut_manager_remove_controller::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn shortcut_manager_add_controller<T: ShortcutManagerImpl>(
    shortcut_manager: *mut ffi::GtkShortcutManager,
    controller: *mut ffi::GtkShortcutController,
) {
    let instance = &*(shortcut_manager as *mut T::Instance);
    let imp = instance.impl_();

    imp.add_controller(
        from_glib_borrow::<_, ShortcutManager>(shortcut_manager).unsafe_cast_ref(),
        &ShortcutController::from_glib_borrow(controller),
    )
}

unsafe extern "C" fn shortcut_manager_remove_controller<T: ShortcutManagerImpl>(
    shortcut_manager: *mut ffi::GtkShortcutManager,
    controller: *mut ffi::GtkShortcutController,
) {
    let instance = &*(shortcut_manager as *mut T::Instance);
    let imp = instance.impl_();

    imp.remove_controller(
        from_glib_borrow::<_, ShortcutManager>(shortcut_manager).unsafe_cast_ref(),
        &ShortcutController::from_glib_borrow(controller),
    )
}
