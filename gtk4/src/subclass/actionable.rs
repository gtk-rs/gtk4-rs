// Take a look at the license at the top of the repository in the LICENSE file.

use super::widget::WidgetImpl;
use crate::Actionable;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString, Variant};

pub trait ActionableImpl: WidgetImpl {
    fn action_name(&self, actionable: &Self::Type) -> Option<GString>;
    fn action_target_value(&self, actionable: &Self::Type) -> Option<Variant>;
    fn set_action_name(&self, actionable: &Self::Type, name: Option<&str>);
    fn set_action_target_value(&self, actionable: &Self::Type, value: Option<&Variant>);
}

pub trait ActionableImplExt: ObjectSubclass {
    fn parent_action_name(&self, actionable: &Self::Type) -> Option<GString>;
    fn parent_action_target_value(&self, actionable: &Self::Type) -> Option<Variant>;
    fn parent_set_action_name(&self, actionable: &Self::Type, name: Option<&str>);
    fn parent_set_action_target_value(&self, actionable: &Self::Type, value: Option<&Variant>);
}

impl<T: ActionableImpl> ActionableImplExt for T {
    fn parent_action_name(&self, actionable: &Self::Type) -> Option<GString> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Actionable>()
                as *const ffi::GtkActionableInterface;

            let func = (*parent_iface)
                .get_action_name
                .expect("no parent \"get_action_name\" implementation");

            from_glib_none(func(
                actionable.unsafe_cast_ref::<Actionable>().to_glib_none().0,
            ))
        }
    }

    fn parent_action_target_value(&self, actionable: &Self::Type) -> Option<Variant> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Actionable>()
                as *const ffi::GtkActionableInterface;

            let func = (*parent_iface)
                .get_action_target_value
                .expect("no parent \"get_action_target_value\" implementation");

            from_glib_none(func(
                actionable.unsafe_cast_ref::<Actionable>().to_glib_none().0,
            ))
        }
    }

    fn parent_set_action_name(&self, actionable: &Self::Type, name: Option<&str>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Actionable>()
                as *const ffi::GtkActionableInterface;

            let func = (*parent_iface)
                .set_action_name
                .expect("no parent \"set_action_name\" implementation");

            func(
                actionable.unsafe_cast_ref::<Actionable>().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    fn parent_set_action_target_value(&self, actionable: &Self::Type, value: Option<&Variant>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Actionable>()
                as *const ffi::GtkActionableInterface;

            let func = (*parent_iface)
                .set_action_target_value
                .expect("no parent \"set_action_target_value\" implementation");

            func(
                actionable.unsafe_cast_ref::<Actionable>().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

unsafe impl<T: ActionableImpl> IsImplementable<T> for Actionable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.get_action_name = Some(actionable_get_action_name::<T>);
        iface.get_action_target_value = Some(actionable_get_action_target_value::<T>);
        iface.set_action_name = Some(actionable_set_action_name::<T>);
        iface.set_action_target_value = Some(actionable_set_action_target_value::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn actionable_get_action_name<T: ActionableImpl>(
    actionable: *mut ffi::GtkActionable,
) -> *const libc::c_char {
    let instance = &*(actionable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.action_name(from_glib_borrow::<_, Actionable>(actionable).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn actionable_get_action_target_value<T: ActionableImpl>(
    actionable: *mut ffi::GtkActionable,
) -> *mut glib::ffi::GVariant {
    let instance = &*(actionable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.action_target_value(from_glib_borrow::<_, Actionable>(actionable).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn actionable_set_action_name<T: ActionableImpl>(
    actionable: *mut ffi::GtkActionable,
    name: *const libc::c_char,
) {
    let instance = &*(actionable as *mut T::Instance);
    let imp = instance.get_impl();
    let name: Borrowed<Option<GString>> = from_glib_borrow(name);
    imp.set_action_name(
        from_glib_borrow::<_, Actionable>(actionable).unsafe_cast_ref(),
        name.as_ref().as_ref().map(|s| s.as_str()),
    )
}

unsafe extern "C" fn actionable_set_action_target_value<T: ActionableImpl>(
    actionable: *mut ffi::GtkActionable,
    value: *mut glib::ffi::GVariant,
) {
    let instance = &*(actionable as *mut T::Instance);
    let imp = instance.get_impl();
    let val: Borrowed<Option<Variant>> = from_glib_borrow(value);

    imp.set_action_target_value(
        from_glib_borrow::<_, Actionable>(actionable).unsafe_cast_ref(),
        val.as_ref().as_ref(),
    )
}
