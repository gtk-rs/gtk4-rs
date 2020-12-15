// Take a look at the license at the top of the repository in the LICENSE file.

use glib::object::{Cast, IsA};
use glib::translate::*;

use crate::{TreeModel, TreeModelFilter, TreePath};

impl TreeModelFilter {
    pub fn new<T: IsA<TreeModel>>(child_model: &T, root: Option<&TreePath>) -> TreeModelFilter {
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
