// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{translate::*, value::FromValue};

use crate::{TreeIter, TreeModel, ffi, prelude::*};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of
/// [`TreeModel`](crate::TreeModel).
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait TreeModelExtManual: IsA<TreeModel> + 'static {
    #[doc(alias = "gtk_tree_model_get")]
    #[doc(alias = "gtk_tree_model_get_value")]
    #[doc(alias = "gtk_tree_model_get_valist")]
    fn get_value(&self, iter: &TreeIter, column: i32) -> glib::Value {
        let total_columns = self.as_ref().n_columns();
        assert!(
            column < total_columns,
            "TreeModel has {total_columns} columns but TreeModelExt::get got {column} passed as a column number",
        );
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::gtk_tree_model_get_value(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                column,
                value.to_glib_none_mut().0,
            );
            value
        }
    }

    #[doc(alias = "gtk_tree_model_get")]
    #[doc(alias = "gtk_tree_model_get_value")]
    #[doc(alias = "gtk_tree_model_get_valist")]
    // rustdoc-stripper-ignore-next
    /// Similar to [`Self::get_value`] but panics if the value is of a different
    /// type.
    fn get<V: for<'b> FromValue<'b> + 'static>(&self, iter: &TreeIter, column: i32) -> V {
        let value = self.get_value(iter, column);
        value
            .get_owned::<V>()
            .expect("Failed to get TreeModel value")
    }

    // rustdoc-stripper-ignore-next
    /// Manual implementation of iter_next that takes a mutable TreeIter.
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_iter_next")]
    fn iter_next(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_next(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
            ))
        }
    }
}

impl<O: IsA<TreeModel>> TreeModelExtManual for O {}
