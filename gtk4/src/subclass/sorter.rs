// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Sorter`](crate::Sorter).

use glib::{translate::*, Object};

use crate::{ffi, prelude::*, subclass::prelude::*, Ordering, Sorter, SorterOrder};

pub trait SorterImpl: SorterImplExt + ObjectImpl {
    fn compare(&self, item1: &Object, item2: &Object) -> Ordering {
        self.parent_compare(item1, item2)
    }
    #[doc(alias = "get_order")]
    fn order(&self) -> SorterOrder {
        self.parent_order()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::SorterImplExt> Sealed for T {}
}

pub trait SorterImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_compare(&self, item1: &Object, item2: &Object) -> Ordering {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkSorterClass;
            let f = (*parent_class)
                .compare
                .expect("No parent class impl for \"compare\"");
            from_glib(f(
                self.obj().unsafe_cast_ref::<Sorter>().to_glib_none().0,
                item1.to_glib_none().0,
                item2.to_glib_none().0,
            ))
        }
    }

    fn parent_order(&self) -> SorterOrder {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkSorterClass;
            let f = (*parent_class)
                .get_order
                .expect("No parent class impl for \"get_order\"");
            from_glib(f(self.obj().unsafe_cast_ref::<Sorter>().to_glib_none().0))
        }
    }
}

impl<T: SorterImpl> SorterImplExt for T {}

unsafe impl<T: SorterImpl> IsSubclassable<T> for Sorter {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();

        let klass = class.as_mut();
        klass.compare = Some(sorter_compare::<T>);
        klass.get_order = Some(sorter_get_order::<T>);
    }
}

unsafe extern "C" fn sorter_compare<T: SorterImpl>(
    ptr: *mut ffi::GtkSorter,
    item1ptr: *mut glib::gobject_ffi::GObject,
    item2ptr: *mut glib::gobject_ffi::GObject,
) -> ffi::GtkOrdering {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.compare(&from_glib_borrow(item1ptr), &from_glib_borrow(item2ptr))
        .into_glib()
}

unsafe extern "C" fn sorter_get_order<T: SorterImpl>(
    ptr: *mut ffi::GtkSorter,
) -> ffi::GtkSorterOrder {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.order().into_glib()
}
