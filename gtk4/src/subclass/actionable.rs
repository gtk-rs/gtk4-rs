// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Actionable`](crate::Actionable)
//! interface.

use glib::{translate::*, GString, Variant};

use crate::{ffi, prelude::*, subclass::prelude::*, Actionable};

pub trait ActionableImpl: WidgetImpl {
    #[doc(alias = "get_action_name")]
    fn action_name(&self) -> Option<GString>;
    #[doc(alias = "get_action_target_value")]
    fn action_target_value(&self) -> Option<Variant>;
    fn set_action_name(&self, name: Option<&str>);
    fn set_action_target_value(&self, value: Option<&Variant>);
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::ActionableImplExt> Sealed for T {}
}

pub trait ActionableImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_action_name(&self) -> Option<GString> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Actionable>()
                as *const ffi::GtkActionableInterface;

            let func = (*parent_iface)
                .get_action_name
                .expect("no parent \"get_action_name\" implementation");

            from_glib_none(func(
                self.obj().unsafe_cast_ref::<Actionable>().to_glib_none().0,
            ))
        }
    }

    fn parent_action_target_value(&self) -> Option<Variant> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Actionable>()
                as *const ffi::GtkActionableInterface;

            let func = (*parent_iface)
                .get_action_target_value
                .expect("no parent \"get_action_target_value\" implementation");

            from_glib_none(func(
                self.obj().unsafe_cast_ref::<Actionable>().to_glib_none().0,
            ))
        }
    }

    fn parent_set_action_name(&self, name: Option<&str>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Actionable>()
                as *const ffi::GtkActionableInterface;

            let func = (*parent_iface)
                .set_action_name
                .expect("no parent \"set_action_name\" implementation");

            func(
                self.obj().unsafe_cast_ref::<Actionable>().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    fn parent_set_action_target_value(&self, value: Option<&Variant>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Actionable>()
                as *const ffi::GtkActionableInterface;

            let func = (*parent_iface)
                .set_action_target_value
                .expect("no parent \"set_action_target_value\" implementation");

            func(
                self.obj().unsafe_cast_ref::<Actionable>().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

impl<T: ActionableImpl> ActionableImplExt for T {}

unsafe impl<T: ActionableImpl> IsImplementable<T> for Actionable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.get_action_name = Some(actionable_get_action_name::<T>);
        iface.get_action_target_value = Some(actionable_get_action_target_value::<T>);
        iface.set_action_name = Some(actionable_set_action_name::<T>);
        iface.set_action_target_value = Some(actionable_set_action_target_value::<T>);
    }
}

unsafe extern "C" fn actionable_get_action_name<T: ActionableImpl>(
    actionable: *mut ffi::GtkActionable,
) -> *const libc::c_char {
    let instance = &*(actionable as *mut T::Instance);
    let imp = instance.imp();

    imp.action_name().into_glib_ptr()
}

unsafe extern "C" fn actionable_get_action_target_value<T: ActionableImpl>(
    actionable: *mut ffi::GtkActionable,
) -> *mut glib::ffi::GVariant {
    let instance = &*(actionable as *mut T::Instance);
    let imp = instance.imp();

    imp.action_target_value().into_glib_ptr()
}

unsafe extern "C" fn actionable_set_action_name<T: ActionableImpl>(
    actionable: *mut ffi::GtkActionable,
    name: *const libc::c_char,
) {
    let instance = &*(actionable as *mut T::Instance);
    let imp = instance.imp();
    let name: Borrowed<Option<GString>> = from_glib_borrow(name);
    imp.set_action_name(name.as_ref().as_ref().map(|s| s.as_str()))
}

unsafe extern "C" fn actionable_set_action_target_value<T: ActionableImpl>(
    actionable: *mut ffi::GtkActionable,
    value: *mut glib::ffi::GVariant,
) {
    let instance = &*(actionable as *mut T::Instance);
    let imp = instance.imp();
    let val: Borrowed<Option<Variant>> = from_glib_borrow(value);

    imp.set_action_target_value(val.as_ref().as_ref())
}
