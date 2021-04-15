// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TreeView;
use glib::translate::*;
use glib::IsA;

pub trait TreeViewExtManual: 'static {
    #[doc(alias = "gtk_tree_view_set_row_separator_func")]
    fn unset_row_separator_func(&self);
}

impl<O: IsA<TreeView>> TreeViewExtManual for O {
    fn unset_row_separator_func(&self) {
        unsafe {
            ffi::gtk_tree_view_set_row_separator_func(
                self.as_ref().to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            );
        }
    }
}
