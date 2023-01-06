// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::boxed::Box as Box_;

use crate::{prelude::*, TreeIter, TreeModel, TreeModelFilter, TreePath};

impl TreeModelFilter {
    #[doc(alias = "gtk_tree_model_filter_new")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn new(child_model: &impl IsA<TreeModel>, root: Option<&TreePath>) -> Self {
        skip_assert_initialized!();
        unsafe {
            TreeModel::from_glib_none(ffi::gtk_tree_model_filter_new(
                child_model.as_ref().to_glib_none().0,
                mut_override(root.to_glib_none().0),
            ))
            .unsafe_cast()
        }
    }
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`TreeModelFilter`](crate::TreeModelFilter).
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait TreeModelFilterExtManual: 'static {
    #[doc(alias = "gtk_tree_model_filter_set_modify_func")]
    fn set_modify_func<F: Fn(&TreeModel, &TreeIter, i32) -> glib::Value + 'static>(
        &self,
        types: &[glib::Type],
        func: F,
    );
}

impl<O: IsA<TreeModelFilter>> TreeModelFilterExtManual for O {
    #[doc(alias = "gtk_tree_model_filter_set_modify_func")]
    fn set_modify_func<F: Fn(&TreeModel, &TreeIter, i32) -> glib::Value + 'static>(
        &self,
        types: &[glib::Type],
        func: F,
    ) {
        unsafe {
            let types_ptr: Vec<glib::ffi::GType> = types.iter().map(|t| t.into_glib()).collect();

            unsafe extern "C" fn func_trampoline<
                F: Fn(&TreeModel, &TreeIter, i32) -> glib::Value + 'static,
            >(
                model: *mut ffi::GtkTreeModel,
                iter: *mut ffi::GtkTreeIter,
                value: *mut glib::gobject_ffi::GValue,
                column: i32,
                user_data: glib::ffi::gpointer,
            ) {
                let f: &F = &*(user_data as *const F);
                let ret = f(&from_glib_borrow(model), &from_glib_borrow(iter), column);
                *value = ret.into_raw();
            }

            unsafe extern "C" fn destroy_func<
                F: Fn(&TreeModel, &TreeIter, i32) -> glib::Value + 'static,
            >(
                user_data: glib::ffi::gpointer,
            ) {
                let _callback: Box_<Option<Box_<F>>> = Box_::from_raw(user_data as *mut _);
            }
            let callback_data: Box_<F> = Box_::new(func);

            ffi::gtk_tree_model_filter_set_modify_func(
                self.as_ref().to_glib_none().0,
                types.len() as i32,
                mut_override(types_ptr.as_ptr()),
                Some(func_trampoline::<F> as _),
                Box_::into_raw(callback_data) as *mut _,
                Some(destroy_func::<F> as _),
            )
        }
    }
}
