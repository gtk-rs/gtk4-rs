// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`CellAreaContext`](crate::CellAreaContext).

use crate::{prelude::*, subclass::prelude::*, CellAreaContext};
use glib::translate::*;
use std::mem::MaybeUninit;

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait CellAreaContextImpl: CellAreaContextImplExt + ObjectImpl {
    fn reset(&self) {
        self.parent_reset()
    }

    fn preferred_height_for_width(&self, width: i32) -> (i32, i32) {
        self.parent_preferred_height_for_width(width)
    }

    fn preferred_width_for_height(&self, height: i32) -> (i32, i32) {
        self.parent_preferred_width_for_height(height)
    }

    fn allocate(&self, width: i32, height: i32) {
        self.parent_allocate(width, height)
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait CellAreaContextImplExt: ObjectSubclass {
    fn parent_reset(&self);
    fn parent_preferred_height_for_width(&self, width: i32) -> (i32, i32);
    fn parent_preferred_width_for_height(&self, height: i32) -> (i32, i32);
    fn parent_allocate(&self, width: i32, height: i32);
}

impl<T: CellAreaContextImpl> CellAreaContextImplExt for T {
    fn parent_reset(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaContextClass;
            if let Some(f) = (*parent_class).reset {
                f(self
                    .obj()
                    .unsafe_cast_ref::<CellAreaContext>()
                    .to_glib_none()
                    .0)
            }
        }
    }

    fn parent_preferred_height_for_width(&self, width: i32) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaContextClass;
            if let Some(f) = (*parent_class).get_preferred_height_for_width {
                let mut minimum_size = MaybeUninit::uninit();
                let mut natural_size = MaybeUninit::uninit();
                f(
                    self.obj()
                        .unsafe_cast_ref::<CellAreaContext>()
                        .to_glib_none()
                        .0,
                    width,
                    minimum_size.as_mut_ptr(),
                    natural_size.as_mut_ptr(),
                );
                (minimum_size.assume_init(), natural_size.assume_init())
            } else {
                (-1, -1)
            }
        }
    }

    fn parent_preferred_width_for_height(&self, height: i32) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaContextClass;
            if let Some(f) = (*parent_class).get_preferred_width_for_height {
                let mut minimum_size = MaybeUninit::uninit();
                let mut natural_size = MaybeUninit::uninit();
                f(
                    self.obj()
                        .unsafe_cast_ref::<CellAreaContext>()
                        .to_glib_none()
                        .0,
                    height,
                    minimum_size.as_mut_ptr(),
                    natural_size.as_mut_ptr(),
                );
                (minimum_size.assume_init(), natural_size.assume_init())
            } else {
                (-1, -1)
            }
        }
    }

    fn parent_allocate(&self, width: i32, height: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaContextClass;
            if let Some(f) = (*parent_class).allocate {
                f(
                    self.obj()
                        .unsafe_cast_ref::<CellAreaContext>()
                        .to_glib_none()
                        .0,
                    width,
                    height,
                )
            }
        }
    }
}

unsafe impl<T: CellAreaContextImpl> IsSubclassable<T> for CellAreaContext {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();

        let klass = class.as_mut();
        klass.reset = Some(cell_area_context_reset::<T>);
        klass.get_preferred_height_for_width =
            Some(cell_area_context_get_preferred_height_for_width::<T>);
        klass.get_preferred_width_for_height =
            Some(cell_area_context_get_preferred_width_for_height::<T>);
        klass.allocate = Some(cell_area_context_allocate::<T>);
    }
}

unsafe extern "C" fn cell_area_context_reset<T: CellAreaContextImpl>(
    ptr: *mut ffi::GtkCellAreaContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.reset()
}

unsafe extern "C" fn cell_area_context_get_preferred_height_for_width<T: CellAreaContextImpl>(
    ptr: *mut ffi::GtkCellAreaContext,
    width: i32,
    minimum_height: *mut libc::c_int,
    natural_height: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let (min_height, nat_height) = imp.preferred_height_for_width(width);
    *minimum_height = min_height;
    *natural_height = nat_height;
}

unsafe extern "C" fn cell_area_context_get_preferred_width_for_height<T: CellAreaContextImpl>(
    ptr: *mut ffi::GtkCellAreaContext,
    height: i32,
    minimum_width: *mut libc::c_int,
    natural_width: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let (min_width, nat_width) = imp.preferred_width_for_height(height);
    *minimum_width = min_width;
    *natural_width = nat_width;
}

unsafe extern "C" fn cell_area_context_allocate<T: CellAreaContextImpl>(
    ptr: *mut ffi::GtkCellAreaContext,
    width: i32,
    height: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.allocate(width, height)
}
