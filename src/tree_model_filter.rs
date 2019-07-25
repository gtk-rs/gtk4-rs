// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::{Cast, IsA};
use glib::translate::*;
use gtk_sys;

use {TreeModel, TreeModelFilter, TreePath};

impl TreeModelFilter {
    pub fn new<T: IsA<TreeModel>>(child_model: &T, root: Option<&TreePath>) -> TreeModelFilter {
        skip_assert_initialized!();
        unsafe {
            TreeModel::from_glib_none(gtk_sys::gtk_tree_model_filter_new(
                child_model.as_ref().to_glib_none().0,
                mut_override(root.to_glib_none().0),
            ))
            .unsafe_cast()
        }
    }
}
