// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`TreeModelFilter`](crate::TreeModelFilter).

use crate::{prelude::*, subclass::prelude::*, TreeIter, TreeModel, TreeModelFilter};
use glib::{translate::*, Value};

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait TreeModelFilterImpl: TreeModelFilterImplExt + ObjectImpl {
    fn visible<M: IsA<TreeModel>>(&self, child_model: &M, iter: &TreeIter) -> bool {
        self.parent_visible(child_model, iter)
    }

    fn modify<M: IsA<TreeModel>>(
        &self,
        child_model: &M,
        iter: &TreeIter,
        value: Value,
        column: i32,
    ) {
        self.parent_modify(child_model, iter, value, column)
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait TreeModelFilterImplExt: ObjectSubclass {
    fn parent_visible<M: IsA<TreeModel>>(&self, child_model: &M, iter: &TreeIter) -> bool;

    fn parent_modify<M: IsA<TreeModel>>(
        &self,
        child_model: &M,
        iter: &TreeIter,
        value: Value,
        index: i32,
    );
}

impl<T: TreeModelFilterImpl> TreeModelFilterImplExt for T {
    fn parent_visible<M: IsA<TreeModel>>(&self, child_model: &M, iter: &TreeIter) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeModelFilterClass;
            if let Some(f) = (*parent_class).visible {
                from_glib(f(
                    self.obj()
                        .unsafe_cast_ref::<TreeModelFilter>()
                        .to_glib_none()
                        .0,
                    child_model.as_ref().to_glib_none().0,
                    iter.to_glib_none().0 as *mut _,
                ))
            } else {
                true // always visible if not set
            }
        }
    }

    fn parent_modify<M: IsA<TreeModel>>(
        &self,
        child_model: &M,
        iter: &TreeIter,
        value: Value,
        index: i32,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeModelFilterClass;
            if let Some(f) = (*parent_class).modify {
                f(
                    self.obj()
                        .unsafe_cast_ref::<TreeModelFilter>()
                        .to_glib_none()
                        .0,
                    child_model.as_ref().to_glib_none().0,
                    iter.to_glib_none().0 as *mut _,
                    value.to_glib_none().0 as *mut _,
                    index,
                )
            }
        }
    }
}

unsafe impl<T: TreeModelFilterImpl> IsSubclassable<T> for TreeModelFilter {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();

        let klass = class.as_mut();
        klass.visible = Some(tree_model_filter_visible::<T>);
        klass.modify = Some(tree_model_filter_modify::<T>);
    }
}

unsafe extern "C" fn tree_model_filter_visible<T: TreeModelFilterImpl>(
    ptr: *mut ffi::GtkTreeModelFilter,
    child_modelptr: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let child_model: Borrowed<TreeModel> = from_glib_borrow(child_modelptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.visible(&*child_model, &iter).into_glib()
}

unsafe extern "C" fn tree_model_filter_modify<T: TreeModelFilterImpl>(
    ptr: *mut ffi::GtkTreeModelFilter,
    child_modelptr: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
    valueptr: *mut glib::gobject_ffi::GValue,
    column: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let child_model: Borrowed<TreeModel> = from_glib_borrow(child_modelptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);
    let value: Value = from_glib_full(valueptr);

    imp.modify(&*child_model, &iter, value, column)
}
