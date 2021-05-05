// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{TreeModel, TreeModelSort};
use glib::object::IsA;
use glib::translate::*;

impl TreeModelSort {
    #[doc(alias = "gtk_tree_model_sort_new_with_model")]
    pub fn new<T: IsA<TreeModel>>(child_model: &T) -> Self {
        skip_assert_initialized!();
        unsafe {
            Self::from_glib_none(ffi::gtk_tree_model_sort_new_with_model(
                child_model.as_ref().to_glib_none().0,
            ))
        }
    }
}
