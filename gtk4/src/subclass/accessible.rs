// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Acessible`](crate::Acessible) interface.

use crate::{prelude::*, subclass::prelude::*, ATContext, Accessible, AccessiblePlatformState};
use glib::translate::*;
use std::mem::MaybeUninit;

pub trait AccessibleImpl: ObjectImpl {
    #[doc(alias = "get_platform_state")]
    fn platform_state(&self, state: AccessiblePlatformState) -> bool {
        self.parent_platform_state(state)
    }

    #[doc(alias = "get_bounds")]
    fn bounds(&self) -> Option<(i32, i32, i32, i32)> {
        self.parent_bounds()
    }

    #[doc(alias = "get_at_context")]
    fn at_context(&self) -> Option<ATContext> {
        self.parent_at_context()
    }

    #[doc(alias = "get_accessible_parent")]
    fn accessible_parent(&self) -> Option<Accessible> {
        self.parent_accessible_parent()
    }

    #[doc(alias = "get_first_accessible_child")]
    fn first_accessible_child(&self) -> Option<Accessible> {
        self.parent_first_accessible_child()
    }

    #[doc(alias = "get_next_accessible_sibling")]
    fn next_accessible_sibling(&self) -> Option<Accessible> {
        self.parent_next_accessible_sibling()
    }
}

pub trait AccessibleImplExt: ObjectSubclass {
    fn parent_platform_state(&self, state: AccessiblePlatformState) -> bool;
    fn parent_bounds(&self) -> Option<(i32, i32, i32, i32)>;
    fn parent_at_context(&self) -> Option<ATContext>;
    fn parent_accessible_parent(&self) -> Option<Accessible>;
    fn parent_first_accessible_child(&self) -> Option<Accessible>;
    fn parent_next_accessible_sibling(&self) -> Option<Accessible>;
}

impl<T: AccessibleImpl> AccessibleImplExt for T {
    fn parent_platform_state(&self, state: AccessiblePlatformState) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Accessible>()
                as *const ffi::GtkAccessibleInterface;

            let func = (*parent_iface)
                .get_platform_state
                .expect("no parent \"get_platform_state\" implementation");

            from_glib(func(
                self.obj().unsafe_cast_ref::<Accessible>().to_glib_none().0,
                state.into_glib(),
            ))
        }
    }

    fn parent_bounds(&self) -> Option<(i32, i32, i32, i32)> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Accessible>()
                as *const ffi::GtkAccessibleInterface;

            let func = (*parent_iface)
                .get_bounds
                .expect("no parent \"get_bounds\" implementation");

            let mut x = MaybeUninit::uninit();
            let mut y = MaybeUninit::uninit();
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();
            let res = from_glib(func(
                self.obj().unsafe_cast_ref::<Accessible>().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            ));
            if res {
                Some((
                    x.assume_init(),
                    y.assume_init(),
                    width.assume_init(),
                    height.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    fn parent_at_context(&self) -> Option<ATContext> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Accessible>()
                as *const ffi::GtkAccessibleInterface;

            let func = (*parent_iface)
                .get_at_context
                .expect("no parent \"get_at_context\" implementation");

            from_glib_full(func(
                self.obj().unsafe_cast_ref::<Accessible>().to_glib_none().0,
            ))
        }
    }

    fn parent_accessible_parent(&self) -> Option<Accessible> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Accessible>()
                as *const ffi::GtkAccessibleInterface;

            let func = (*parent_iface)
                .get_accessible_parent
                .expect("no parent \"get_accessible_parent\" implementation");

            from_glib_full(func(
                self.obj().unsafe_cast_ref::<Accessible>().to_glib_none().0,
            ))
        }
    }

    fn parent_first_accessible_child(&self) -> Option<Accessible> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Accessible>()
                as *const ffi::GtkAccessibleInterface;

            let func = (*parent_iface)
                .get_first_accessible_child
                .expect("no parent \"get_first_accessible_child\" implementation");

            from_glib_full(func(
                self.obj().unsafe_cast_ref::<Accessible>().to_glib_none().0,
            ))
        }
    }

    fn parent_next_accessible_sibling(&self) -> Option<Accessible> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Accessible>()
                as *const ffi::GtkAccessibleInterface;

            let func = (*parent_iface)
                .get_next_accessible_sibling
                .expect("no parent \"get_next_accessible_sibling\" implementation");

            from_glib_full(func(
                self.obj().unsafe_cast_ref::<Accessible>().to_glib_none().0,
            ))
        }
    }
}

unsafe impl<T: AccessibleImpl> IsImplementable<T> for Accessible {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.get_platform_state = Some(accessible_get_platform_state::<T>);
        iface.get_bounds = Some(accessible_get_bounds::<T>);
        iface.get_at_context = Some(accessible_get_at_context::<T>);
        iface.get_accessible_parent = Some(accessible_get_accessible_parent::<T>);
        iface.get_first_accessible_child = Some(accessible_get_first_accessible_child::<T>);
        iface.get_next_accessible_sibling = Some(accessible_get_next_accessible_sibling::<T>);
    }
}

unsafe extern "C" fn accessible_get_platform_state<T: AccessibleImpl>(
    accessible: *mut ffi::GtkAccessible,
    state: ffi::GtkAccessiblePlatformState,
) -> glib::ffi::gboolean {
    let instance = &*(accessible as *mut T::Instance);
    let imp = instance.imp();

    imp.platform_state(from_glib(state)).into_glib()
}

unsafe extern "C" fn accessible_get_bounds<T: AccessibleImpl>(
    accessible: *mut ffi::GtkAccessible,
    xptr: *mut libc::c_int,
    yptr: *mut libc::c_int,
    widthptr: *mut libc::c_int,
    heightptr: *mut libc::c_int,
) -> glib::ffi::gboolean {
    let instance = &*(accessible as *mut T::Instance);
    let imp = instance.imp();

    if let Some((x, y, width, height)) = imp.bounds() {
        *xptr = x;
        *yptr = y;
        *widthptr = width;
        *heightptr = height;

        true.into_glib()
    } else {
        false.into_glib()
    }
}

unsafe extern "C" fn accessible_get_at_context<T: AccessibleImpl>(
    accessible: *mut ffi::GtkAccessible,
) -> *mut ffi::GtkATContext {
    let instance = &*(accessible as *mut T::Instance);
    let imp = instance.imp();

    imp.at_context().into_glib_ptr()
}

unsafe extern "C" fn accessible_get_accessible_parent<T: AccessibleImpl>(
    accessible: *mut ffi::GtkAccessible,
) -> *mut ffi::GtkAccessible {
    let instance = &*(accessible as *mut T::Instance);
    let imp = instance.imp();

    imp.accessible_parent().into_glib_ptr()
}

unsafe extern "C" fn accessible_get_first_accessible_child<T: AccessibleImpl>(
    accessible: *mut ffi::GtkAccessible,
) -> *mut ffi::GtkAccessible {
    let instance = &*(accessible as *mut T::Instance);
    let imp = instance.imp();

    imp.first_accessible_child().into_glib_ptr()
}

unsafe extern "C" fn accessible_get_next_accessible_sibling<T: AccessibleImpl>(
    accessible: *mut ffi::GtkAccessible,
) -> *mut ffi::GtkAccessible {
    let instance = &*(accessible as *mut T::Instance);
    let imp = instance.imp();

    imp.next_accessible_sibling().into_glib_ptr()
}
