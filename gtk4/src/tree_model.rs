// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::TreeModelExt;
use crate::{TreeIter, TreeModel};
use glib::translate::*;
use glib::IsA;

pub trait TreeModelExtManual: 'static {
    #[doc(alias = "gtk_tree_model_get")]
    #[doc(alias = "gtk_tree_model_get_value")]
    #[doc(alias = "gtk_tree_model_get_valist")]
    fn get(&self, iter: &TreeIter, column: i32) -> glib::Value;
}

impl<O: IsA<TreeModel>> TreeModelExtManual for O {
    fn get(&self, iter: &TreeIter, column: i32) -> glib::Value {
        let total_columns = self.as_ref().get_n_columns();
        assert!(
            column < total_columns,
            "TreeModel has {} columns but TreeModelExt::get got {} passed as a column number",
            total_columns,
            column
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
}
