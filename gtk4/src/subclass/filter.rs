// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Filter`](crate::Filter).

use crate::subclass::prelude::*;
use crate::{Filter, FilterMatch};
use glib::translate::*;
use glib::{Cast, Object};

pub trait FilterImpl: FilterImplExt + ObjectImpl {
    #[doc(alias = "get_strictness")]
    fn strictness(&self, filter: &Self::Type) -> FilterMatch {
        self.parent_strictness(filter)
    }
    fn match_(&self, filter: &Self::Type, item: &Object) -> bool {
        self.parent_match_(filter, item)
    }
}

pub trait FilterImplExt: ObjectSubclass {
    fn parent_strictness(&self, filter: &Self::Type) -> FilterMatch;
    fn parent_match_(&self, filter: &Self::Type, item: &Object) -> bool;
}

impl<T: FilterImpl> FilterImplExt for T {
    fn parent_strictness(&self, filter: &Self::Type) -> FilterMatch {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkFilterClass;
            let f = (*parent_class)
                .get_strictness
                .expect("No parent class impl for \"get_strictness\"");
            from_glib(f(filter.unsafe_cast_ref::<Filter>().to_glib_none().0))
        }
    }

    fn parent_match_(&self, filter: &Self::Type, item: &Object) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkFilterClass;
            let f = (*parent_class)
                .match_
                .expect("No parent class impl for \"match\"");
            from_glib(f(
                filter.unsafe_cast_ref::<Filter>().to_glib_none().0,
                item.to_glib_none().0,
            ))
        }
    }
}

unsafe impl<T: FilterImpl> IsSubclassable<T> for Filter {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert!(
            crate::rt::is_initialized(),
            "GTK has to be initialized first"
        );

        let klass = class.as_mut();
        klass.match_ = Some(filter_match::<T>);
        klass.get_strictness = Some(filter_get_strictness::<T>);
    }
}

unsafe extern "C" fn filter_get_strictness<T: FilterImpl>(
    ptr: *mut ffi::GtkFilter,
) -> ffi::GtkFilterMatch {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Filter> = from_glib_borrow(ptr);

    imp.strictness(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn filter_match<T: FilterImpl>(
    ptr: *mut ffi::GtkFilter,
    itemptr: *mut glib::gobject_ffi::GObject,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Filter> = from_glib_borrow(ptr);

    imp.match_(wrap.unsafe_cast_ref(), &from_glib_borrow(itemptr))
        .into_glib()
}
