// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::{SortType, TreeIter, TreeModel, TreeSortable};
use glib::translate::*;
use glib::Cast;

#[derive(Debug)]
pub struct TreeIterCompareCallback {
    compare_func: ffi::GtkTreeIterCompareFunc,
    user_data: glib::ffi::gpointer,
    destroy_notify: glib::ffi::GDestroyNotify,
}

impl TreeIterCompareCallback {
    pub fn call(
        &self,
        tree_model: &TreeModel,
        iter_a: &TreeIter,
        iter_b: &TreeIter,
    ) -> std::cmp::Ordering {
        unsafe {
            if let Some(compare_func) = self.compare_func {
                from_glib(compare_func(
                    tree_model.to_glib_none().0,
                    mut_override(iter_a.to_glib_none().0),
                    mut_override(iter_b.to_glib_none().0),
                    self.user_data,
                ))
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}

impl Drop for TreeIterCompareCallback {
    fn drop(&mut self) {
        unsafe {
            if let Some(destroy_notify) = self.destroy_notify {
                destroy_notify(self.user_data)
            }
        }
    }
}
pub trait TreeSortableImpl: TreeModelImpl + ObjectImpl {
    fn sort_column_changed(&self, tree_sortable: &Self::Type) {
        self.parent_sort_column_changed(tree_sortable)
    }

    fn sort_column_id(&self, tree_sortable: &Self::Type) -> Option<(i32, SortType)> {
        self.parent_sort_column_id(tree_sortable)
    }

    fn set_sort_column_id(
        &self,
        tree_sortable: &Self::Type,
        sort_column_id: i32,
        sort_type: SortType,
    ) {
        self.parent_set_sort_column_id(tree_sortable, sort_column_id, sort_type)
    }

    fn has_default_sort_func(&self, tree_sortable: &Self::Type) -> bool;

    fn set_sort_func(
        &self,
        tree_sortable: &Self::Type,
        sort_column_id: i32,
        filter_func: Option<&TreeIterCompareCallback>,
    );

    fn set_default_sort_func(
        &self,
        tree_sortable: &Self::Type,
        filter_func: Option<&TreeIterCompareCallback>,
    );
}

pub trait TreeSortableImplExt: ObjectSubclass {
    fn parent_sort_column_changed(&self, tree_sortable: &Self::Type);
    fn parent_sort_column_id(&self, tree_sortable: &Self::Type) -> Option<(i32, SortType)>;
    fn parent_set_sort_column_id(
        &self,
        tree_sortable: &Self::Type,
        sort_column_id: i32,
        sort_type: SortType,
    );
}

impl<O: TreeSortableImpl> TreeSortableImplExt for O {
    fn parent_sort_column_changed(&self, tree_sortable: &Self::Type) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TreeSortable>()
                as *const ffi::GtkTreeSortableIface;

            if let Some(f) = (*parent_iface).sort_column_changed.as_ref() {
                f(tree_sortable
                    .unsafe_cast_ref::<TreeSortable>()
                    .to_glib_none()
                    .0);
            }
        }
    }

    fn parent_sort_column_id(&self, tree_sortable: &Self::Type) -> Option<(i32, SortType)> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TreeSortable>()
                as *const ffi::GtkTreeSortableIface;

            let f = (*parent_iface).get_sort_column_id.as_ref().unwrap();
            let sort_column_id = std::ptr::null_mut();
            let sort_type = std::ptr::null_mut();
            let ret = from_glib(f(
                tree_sortable
                    .unsafe_cast_ref::<TreeSortable>()
                    .to_glib_none()
                    .0,
                sort_column_id,
                sort_type,
            ));

            if ret {
                Some((sort_column_id as i32, from_glib(sort_type as i32)))
            } else {
                None
            }
        }
    }

    fn parent_set_sort_column_id(
        &self,
        tree_sortable: &Self::Type,
        sort_column_id: i32,
        sort_type: SortType,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TreeSortable>()
                as *const ffi::GtkTreeSortableIface;

            let f = (*parent_iface).set_sort_column_id.as_ref().unwrap();

            f(
                tree_sortable
                    .unsafe_cast_ref::<TreeSortable>()
                    .to_glib_none()
                    .0,
                sort_column_id,
                sort_type.into_glib(),
            );
        }
    }
}

unsafe impl<T: TreeSortableImpl> IsImplementable<T> for TreeSortable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.sort_column_changed = Some(tree_sortable_sort_column_changed::<T>);
        iface.get_sort_column_id = Some(tree_sortable_get_sort_column_id::<T>);
        iface.set_sort_column_id = Some(tree_sortable_set_sort_column_id::<T>);
        iface.has_default_sort_func = Some(tree_sortable_has_default_sort_func::<T>);
        iface.set_sort_func = Some(tree_sortable_set_sort_func::<T>);
        iface.set_default_sort_func = Some(tree_sortable_set_default_sort_func::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn tree_sortable_sort_column_changed<T: TreeSortableImpl>(
    tree_sortable: *mut ffi::GtkTreeSortable,
) {
    let instance = &*(tree_sortable as *mut T::Instance);
    let imp = instance.impl_();

    imp.sort_column_changed(from_glib_borrow::<_, TreeSortable>(tree_sortable).unsafe_cast_ref())
}

unsafe extern "C" fn tree_sortable_get_sort_column_id<T: TreeSortableImpl>(
    tree_sortable: *mut ffi::GtkTreeSortable,
    sort_column_idptr: *mut libc::c_int,
    sort_typeptr: *mut ffi::GtkSortType,
) -> glib::ffi::gboolean {
    let instance = &*(tree_sortable as *mut T::Instance);
    let imp = instance.impl_();

    let ret =
        imp.sort_column_id(from_glib_borrow::<_, TreeSortable>(tree_sortable).unsafe_cast_ref());

    if let Some((sort_column_id, sort_type)) = ret {
        *sort_column_idptr = sort_column_id;
        *sort_typeptr = sort_type.into_glib();
        true.into_glib()
    } else {
        *sort_column_idptr = *std::ptr::null_mut();
        *sort_typeptr = *std::ptr::null_mut();
        false.into_glib()
    }
}

unsafe extern "C" fn tree_sortable_set_sort_column_id<T: TreeSortableImpl>(
    tree_sortable: *mut ffi::GtkTreeSortable,
    sort_column_id: i32,
    sort_typeptr: ffi::GtkSortType,
) {
    let instance = &*(tree_sortable as *mut T::Instance);
    let imp = instance.impl_();

    imp.set_sort_column_id(
        from_glib_borrow::<_, TreeSortable>(tree_sortable).unsafe_cast_ref(),
        sort_column_id,
        from_glib(sort_typeptr),
    );
}

unsafe extern "C" fn tree_sortable_has_default_sort_func<T: TreeSortableImpl>(
    tree_sortable: *mut ffi::GtkTreeSortable,
) -> glib::ffi::gboolean {
    let instance = &*(tree_sortable as *mut T::Instance);
    let imp = instance.impl_();

    imp.has_default_sort_func(from_glib_borrow::<_, TreeSortable>(tree_sortable).unsafe_cast_ref())
        .into_glib()
}

unsafe extern "C" fn tree_sortable_set_sort_func<T: TreeSortableImpl>(
    tree_sortable: *mut ffi::GtkTreeSortable,
    sort_column_id: i32,
    compare_func: ffi::GtkTreeIterCompareFunc,
    user_data: glib::ffi::gpointer,
    destroy_notify: glib::ffi::GDestroyNotify,
) {
    let instance = &*(tree_sortable as *mut T::Instance);
    let imp = instance.impl_();

    let callback = if compare_func.is_some() {
        None
    } else {
        Some(TreeIterCompareCallback {
            compare_func,
            user_data,
            destroy_notify,
        })
    };

    imp.set_sort_func(
        from_glib_borrow::<_, TreeSortable>(tree_sortable).unsafe_cast_ref(),
        sort_column_id,
        callback.as_ref(),
    );
}

unsafe extern "C" fn tree_sortable_set_default_sort_func<T: TreeSortableImpl>(
    tree_sortable: *mut ffi::GtkTreeSortable,
    compare_func: ffi::GtkTreeIterCompareFunc,
    user_data: glib::ffi::gpointer,
    destroy_notify: glib::ffi::GDestroyNotify,
) {
    let instance = &*(tree_sortable as *mut T::Instance);
    let imp = instance.impl_();

    let callback = if compare_func.is_some() {
        None
    } else {
        Some(TreeIterCompareCallback {
            compare_func,
            user_data,
            destroy_notify,
        })
    };
    imp.set_default_sort_func(
        from_glib_borrow::<_, TreeSortable>(tree_sortable).unsafe_cast_ref(),
        callback.as_ref(),
    );
}
