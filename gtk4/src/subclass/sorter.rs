// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Ordering, Sorter, SorterOrder};
use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Object};

pub trait SorterImpl: SorterImplExt + ObjectImpl {
    fn compare(&self, sorter: &Self::Type, item1: &Object, item2: &Object) -> Ordering {
        self.parent_compare(sorter, item1, item2)
    }
    fn order(&self, sorter: &Self::Type) -> SorterOrder {
        self.parent_order(sorter)
    }
}

pub trait SorterImplExt: ObjectSubclass {
    fn parent_compare(&self, sorter: &Self::Type, item1: &Object, item2: &Object) -> Ordering;
    fn parent_order(&self, sorter: &Self::Type) -> SorterOrder;
}

impl<T: SorterImpl> SorterImplExt for T {
    fn parent_compare(&self, sorter: &Self::Type, item1: &Object, item2: &Object) -> Ordering {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkSorterClass;
            let f = (*parent_class)
                .compare
                .expect("No parent class impl for \"compare\"");
            from_glib(f(
                sorter.unsafe_cast_ref::<Sorter>().to_glib_none().0,
                item1.to_glib_none().0,
                item2.to_glib_none().0,
            ))
        }
    }

    fn parent_order(&self, sorter: &Self::Type) -> SorterOrder {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkSorterClass;
            let f = (*parent_class)
                .get_order
                .expect("No parent class impl for \"get_order\"");
            from_glib(f(sorter.unsafe_cast_ref::<Sorter>().to_glib_none().0))
        }
    }
}

unsafe impl<T: SorterImpl> IsSubclassable<T> for Sorter {
    fn class_init(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.compare = Some(sorter_compare::<T>);
        klass.get_order = Some(sorter_get_order::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn sorter_compare<T: SorterImpl>(
    ptr: *mut ffi::GtkSorter,
    item1ptr: *mut glib::gobject_ffi::GObject,
    item2ptr: *mut glib::gobject_ffi::GObject,
) -> ffi::GtkOrdering {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Sorter> = from_glib_borrow(ptr);

    imp.compare(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(item1ptr),
        &from_glib_borrow(item2ptr),
    )
    .to_glib()
}

unsafe extern "C" fn sorter_get_order<T: SorterImpl>(
    ptr: *mut ffi::GtkSorter,
) -> ffi::GtkSorterOrder {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Sorter> = from_glib_borrow(ptr);

    imp.order(wrap.unsafe_cast_ref()).to_glib()
}
