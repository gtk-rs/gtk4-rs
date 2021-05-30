// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::CellAreaContext;
use glib::translate::*;
use glib::{Cast, Object};
use std::mem::MaybeUninit;

pub trait CellAreaContextImpl: CellAreaContextImplExt + ObjectImpl {
    fn reset(&self, cell_area_context: &Self::Type) {
        self.parent_reset(cell_area_context)
    }

    fn preferred_height_for_width(&self, cell_area_context: &Self::Type, width: i32) -> (i32, i32) {
        self.parent_preferred_height_for_width(cell_area_context, width)
    }

    fn preferred_width_for_height(
        &self,
        cell_area_context: &Self::Type,
        height: i32,
    ) -> (i32, i32) {
        self.parent_preferred_width_for_height(cell_area_context, height)
    }

    fn allocate(&self, cell_area_context: &Self::Type, width: i32, height: i32) {
        self.parent_allocate(cell_area_context, width, height)
    }
}

pub trait CellAreaContextImplExt: ObjectSubclass {
    fn parent_reset(&self, cell_area_context: &Self::Type);
    fn parent_preferred_height_for_width(
        &self,
        cell_area_context: &Self::Type,
        width: i32,
    ) -> (i32, i32);
    fn parent_preferred_width_for_height(
        &self,
        cell_area_context: &Self::Type,
        height: i32,
    ) -> (i32, i32);
    fn parent_allocate(&self, cell_area_context: &Self::Type, width: i32, height: i32);
}

impl<T: CellAreaContextImpl> CellAreaContextImplExt for T {
    fn parent_reset(&self, cell_area_context: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaContextClass;
            if let Some(f) = (*parent_class).reset {
                f(cell_area_context
                    .unsafe_cast_ref::<CellAreaContext>()
                    .to_glib_none()
                    .0)
            }
        }
    }

    fn parent_preferred_height_for_width(
        &self,
        cell_area_context: &Self::Type,
        width: i32,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaContextClass;
            if let Some(f) = (*parent_class).get_preferred_height_for_width {
                let mut minimum_size = MaybeUninit::uninit();
                let mut natural_size = MaybeUninit::uninit();
                f(
                    cell_area_context
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

    fn parent_preferred_width_for_height(
        &self,
        cell_area_context: &Self::Type,
        height: i32,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaContextClass;
            if let Some(f) = (*parent_class).get_preferred_width_for_height {
                let mut minimum_size = MaybeUninit::uninit();
                let mut natural_size = MaybeUninit::uninit();
                f(
                    cell_area_context
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

    fn parent_allocate(&self, cell_area_context: &Self::Type, width: i32, height: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaContextClass;
            if let Some(f) = (*parent_class).allocate {
                f(
                    cell_area_context
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
        <Object as IsSubclassable<T>>::class_init(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        let klass = class.as_mut();
        klass.reset = Some(cell_area_context_reset::<T>);
        klass.get_preferred_height_for_width =
            Some(cell_area_context_get_preferred_height_for_width::<T>);
        klass.get_preferred_width_for_height =
            Some(cell_area_context_get_preferred_width_for_height::<T>);
        klass.allocate = Some(cell_area_context_allocate::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn cell_area_context_reset<T: CellAreaContextImpl>(
    ptr: *mut ffi::GtkCellAreaContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellAreaContext> = from_glib_borrow(ptr);

    imp.reset(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn cell_area_context_get_preferred_height_for_width<T: CellAreaContextImpl>(
    ptr: *mut ffi::GtkCellAreaContext,
    width: i32,
    minimum_height: *mut libc::c_int,
    natural_height: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellAreaContext> = from_glib_borrow(ptr);

    let (min_height, nat_height) = imp.preferred_height_for_width(wrap.unsafe_cast_ref(), width);
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
    let imp = instance.impl_();
    let wrap: Borrowed<CellAreaContext> = from_glib_borrow(ptr);

    let (min_width, nat_width) = imp.preferred_width_for_height(wrap.unsafe_cast_ref(), height);
    *minimum_width = min_width;
    *natural_width = nat_width;
}

unsafe extern "C" fn cell_area_context_allocate<T: CellAreaContextImpl>(
    ptr: *mut ffi::GtkCellAreaContext,
    width: i32,
    height: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellAreaContext> = from_glib_borrow(ptr);

    imp.allocate(wrap.unsafe_cast_ref(), width, height)
}
